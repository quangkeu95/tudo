use std::collections::HashMap;

use daggy::{Dag, EdgeIndex, NodeIndex, Walker};
use futures::stream::StreamExt;
use thiserror::Error;
use tokio::sync::broadcast;
use tudo_config::logging::{__tracing as tracing, error, info, instrument};
use tudo_interpreter::job::JobConfig;
use tudo_interpreter::{
    job::JobName,
    workflow::{WorkflowConfig, WorkflowName},
};

use crate::job::{ExecuteJobError, JobExecutor};
use crate::playbook::SharedMutexPlaybookContext;
use crate::workflow::WorkflowContext;

use super::{SharedMutableWorkflowContext, WorkflowContextError};

pub struct WorkflowExecutor {}

impl WorkflowExecutor {
    #[instrument(name = "WorkflowExecute", skip(workflow_config))]
    pub async fn execute(
        workflow_name: WorkflowName,
        workflow_config: WorkflowConfig,
        playbook_context: SharedMutexPlaybookContext,
    ) -> Result<WorkflowName, WorkflowExecutorError> {
        info!("Executing workflow {:#?}", workflow_name);

        let workflow_context = WorkflowContext::new_share_mutable();

        let mut workflow_dag = WorkflowDAG::new();

        let job_configs = workflow_config.get_jobs();
        let jobs_num = job_configs.len();

        for (job_name, job_config) in job_configs {
            let _node_index = workflow_dag.add_node(job_name, job_config)?;
        }

        for (job_name, job_config) in job_configs {
            for prerequisited_job_name in job_config.prerequisited_jobs() {
                workflow_dag.add_edge(prerequisited_job_name, job_name)?;
            }
        }

        // sorted topological DAG tell us the executing job order
        let sorted_graph = workflow_dag.topological_sort();

        let tasks =
            sorted_graph.into_iter().map(|node| {
                tokio::spawn({
                    let job_name = node.job_name;
                    let job_config = node.job_config;
                    let workflow_context = workflow_context.clone();

                    async move {
                        Self::spawn_job(job_name, job_config, jobs_num, workflow_context).await
                    }
                })
            });

        let mut stream = futures::stream::iter(tasks).buffered(jobs_num);

        // jobs are executed in parallel, so if a job is failed, it should not affect other jobs
        while let Some(job_result) = stream.next().await {
            let job_result = job_result?;

            match job_result {
                Ok(job_name) => {
                    info!("Finish job {:#?}", job_name);
                }
                Err(err) => {
                    error!("error execute job {:#?}", err);
                }
            }
        }

        Ok(workflow_name.clone())
    }

    async fn spawn_job(
        job_name: JobName,
        job_config: JobConfig,
        maximum_num_rx: usize,
        workflow_context: SharedMutableWorkflowContext,
    ) -> Result<JobName, WorkflowExecutorError> {
        let (tx, _rx) = broadcast::channel::<()>(maximum_num_rx);

        let dependencies = job_config
            .prerequisited_jobs()
            .iter()
            .map(|prerequisited_job_name| {
                let workflow_context = workflow_context.clone();

                async move {
                    let workflow_context_mutex = workflow_context.lock().await;
                    workflow_context_mutex
                        .wait_for_job_to_finish(prerequisited_job_name)
                        .await
                }
            });

        // wait for all prerequisited jobs to finish first
        futures::future::join_all(dependencies).await;

        // add pending jobs
        let mut workflow_context_mutex = workflow_context.lock().await;
        workflow_context_mutex.add_pending_job(&job_name, tx.clone())?;

        // execute the job
        let job_context = JobExecutor::execute(&job_name, &job_config).await?;
        workflow_context_mutex.add_job_context(&job_name, job_context)?;

        let _ = tx.send(());

        Ok(job_name.clone())
    }
}

#[derive(Debug)]
struct WorkflowDAG {
    dag: Dag<WorkflowDAGNode, u32, u32>,
    job_to_node_index: HashMap<JobName, NodeIndex<u32>>,
}

impl WorkflowDAG {
    pub fn new() -> Self {
        Self {
            dag: Dag::new(),
            job_to_node_index: HashMap::new(),
        }
    }

    /// Add node to the DAG
    pub fn add_node(
        &mut self,
        job_name: &JobName,
        job_config: &JobConfig,
    ) -> Result<NodeIndex<u32>, WorkflowDAGError> {
        if self.job_to_node_index.contains_key(job_name) {
            return Err(WorkflowDAGError::NodeExisted(job_name.clone()));
        }
        let node_index = self
            .dag
            .add_node(WorkflowDAGNode::new(job_name.clone(), job_config.clone()));
        self.job_to_node_index.insert(job_name.clone(), node_index);
        Ok(node_index)
    }

    /// Add edge to the DAG
    pub fn add_edge(
        &mut self,
        from: &JobName,
        to: &JobName,
    ) -> Result<EdgeIndex<u32>, WorkflowDAGError> {
        if !self.job_to_node_index.contains_key(from) {
            return Err(WorkflowDAGError::NodeNotExisted(from.clone()));
        } else if !self.job_to_node_index.contains_key(to) {
            return Err(WorkflowDAGError::NodeNotExisted(to.clone()));
        }
        let from_index = *self.job_to_node_index.get(from).unwrap();
        let to_index = *self.job_to_node_index.get(to).unwrap();

        let edge_index = self
            .dag
            .add_edge(from_index, to_index, 0)
            .map_err(|e| WorkflowDAGError::CyclicEdge(e.to_string()))?;

        let mut from_node_rank = 0;
        {
            let mut from_node = self.dag.node_weight_mut(from_index).unwrap();

            if from_node.job_rank.is_none() {
                from_node.job_rank = Some(0);
            } else {
                from_node_rank = from_node.job_rank.unwrap();
            }
        }
        let mut to_node = self.dag.node_weight_mut(to_index).unwrap();

        if to_node.job_rank.is_none() || to_node.job_rank.unwrap() <= from_node_rank {
            to_node.job_rank = Some(from_node_rank + 1);
        }

        Ok(edge_index)
    }

    /// Topological sort the DAG
    pub fn topological_sort(&self) -> Vec<WorkflowDAGNode> {
        let node_count = self.dag.node_count();
        let mut visited = vec![false; node_count];
        let mut ordering = vec![0usize; node_count];
        let mut i = node_count - 1;

        self.job_to_node_index
            .iter()
            .for_each(|(_job_name, node_index)| {
                let index = node_index.index();

                if !visited[index] {
                    i = Self::dfs(self, i, index, &mut visited, &mut ordering)
                }
            });

        ordering
            .into_iter()
            .filter_map(|i| self.dag.node_weight(NodeIndex::new(i)).map(Clone::clone))
            .collect::<Vec<WorkflowDAGNode>>()
    }

    fn dfs(&self, mut i: usize, at: usize, visited: &mut [bool], ordering: &mut [usize]) -> usize {
        visited[at] = true;
        let node_index = NodeIndex::new(at);

        let mut children_walker = self.dag.children(node_index);

        while let Some(child_node_index) = children_walker
            .walk_next(&self.dag)
            .map(|(_, n_index)| n_index)
        {
            let child_node_index = child_node_index.index();
            if !visited[child_node_index] {
                i = Self::dfs(self, i, child_node_index, visited, ordering);
            }
        }

        ordering[i] = at;
        if i == 0 {
            0
        } else {
            i - 1
        }
    }
}

#[derive(Debug, Error)]
pub enum WorkflowDAGError {
    #[error("job name {:#?} not existed", .0)]
    JobNameNotExisted(JobName),
    #[error("node {:#?} existed", .0)]
    NodeExisted(JobName),
    #[error("node {:#?} not existed", .0)]
    NodeNotExisted(JobName),
    #[error("cylic edge in DAG, error {:#?}", .0)]
    CyclicEdge(String),
}

#[derive(Debug, Clone)]
struct WorkflowDAGNode {
    job_name: JobName,
    job_config: JobConfig,
    job_rank: Option<u32>,
}

impl WorkflowDAGNode {
    pub fn new(job_name: JobName, job_config: JobConfig) -> Self {
        Self {
            job_name,
            job_config,
            job_rank: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum WorkflowExecutorError {
    #[error(transparent)]
    TokioTaskJoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    WorkflowDAGError(#[from] WorkflowDAGError),
    #[error(transparent)]
    ExecuteJobError(#[from] ExecuteJobError),
    #[error(transparent)]
    WorkflowContextError(#[from] WorkflowContextError),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::playbook::PlaybookContextBuilder;
    use tudo_interpreter::{job::JobConfigBuilder, workflow::WorkflowConfigBuilder};

    use super::*;

    #[tokio::test]
    async fn can_execute_workflow() {
        let workflow_name: WorkflowName = WorkflowName::from_str("test_workflow").unwrap();
        let mut jobs = HashMap::new();

        // scenario
        // job_4 depends on job_1 + job_2
        // job_5 depends on job_1 + job 3
        // job_6 depends on job 3 + job_4 + job_5
        let job_1_name = JobName::from_str("job_1").unwrap();
        let job_2_name = JobName::from_str("job_2").unwrap();
        let job_3_name = JobName::from_str("job_3").unwrap();
        let job_4_name = JobName::from_str("job_4").unwrap();
        let job_5_name = JobName::from_str("job_5").unwrap();
        let job_6_name = JobName::from_str("job_6").unwrap();

        jobs.insert(
            job_1_name.clone(),
            JobConfigBuilder::default()
                .steps(vec![])
                .depends_on(vec![])
                .build()
                .unwrap(),
        );
        jobs.insert(
            job_2_name.clone(),
            JobConfigBuilder::default()
                .steps(vec![])
                .depends_on(vec![])
                .build()
                .unwrap(),
        );
        jobs.insert(
            job_3_name.clone(),
            JobConfigBuilder::default()
                .steps(vec![])
                .depends_on(vec![])
                .build()
                .unwrap(),
        );

        jobs.insert(
            job_4_name.clone(),
            JobConfigBuilder::default()
                .steps(Vec::new())
                .depends_on(vec![job_1_name.clone(), job_2_name.clone()])
                .build()
                .unwrap(),
        );
        jobs.insert(
            job_5_name.clone(),
            JobConfigBuilder::default()
                .steps(Vec::new())
                .depends_on(vec![job_1_name.clone(), job_3_name.clone()])
                .build()
                .unwrap(),
        );
        jobs.insert(
            job_6_name.clone(),
            JobConfigBuilder::default()
                .steps(Vec::new())
                .depends_on(vec![
                    job_3_name.clone(),
                    job_4_name.clone(),
                    job_5_name.clone(),
                ])
                .build()
                .unwrap(),
        );

        let workflow_config = WorkflowConfigBuilder::default().jobs(jobs).build().unwrap();
        let playbook_context = PlaybookContextBuilder::default()
            .build()
            .unwrap()
            .into_shared_mutex();

        WorkflowExecutor::execute(workflow_name, workflow_config, playbook_context)
            .await
            .unwrap();
    }
}
