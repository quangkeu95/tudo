use std::{collections::BTreeMap, error::Error};

use tudo_primitives::Step;

use crate::job::JobContext;

use super::{StepOutput as StepOutputEnum, StepTypes};

#[derive(Debug)]
pub enum PipelineState {
    Continue,
    Stop(Box<dyn Error>),
}

/// Chain multiple [`Step`] and execute them with StepPipeline.
pub struct StepPipeline {
    pub pipeline_state: PipelineState,
    pub step_counter: usize,
}

impl StepPipeline {
    pub fn new() -> Self {
        Self {
            pipeline_state: PipelineState::Continue,
            step_counter: 0,
        }
    }

    /// Execute step and ignore step's result
    pub async fn execute_step<S, StepInput, StepOutput, StepError>(
        &mut self,
        step: S,
        step_input: &StepInput,
    ) where
        S: Step<Input = StepInput, Output = StepOutput, Error = StepError>,
        StepError: std::error::Error + 'static,
    {
        match step.execute(step_input).await {
            Ok(_result) => {
                // ignore Step's result
                self.step_counter += 1;
            }
            Err(error) => self.pipeline_state = PipelineState::Stop(Box::new(error)),
        }
    }

    /// Execute step and save step's output into job context
    pub async fn execute_step_with_context<S, StepInput, StepOutput, StepError>(
        &mut self,
        step: S,
        step_input: &StepInput,
        job_context: &mut JobContext,
    ) where
        S: Step<Input = StepInput, Output = StepOutput, Error = StepError>,
        StepOutput: Into<StepOutputEnum>,
        StepError: std::error::Error + 'static,
    {
        match step.execute(step_input).await {
            Ok(result) => {
                let step_id = step.id().unwrap_or(self.step_counter.to_string());

                if let Err(error) = job_context.add_step_output(step_id, result.into()) {
                    self.pipeline_state = PipelineState::Stop(Box::new(error));
                    return;
                }
                self.step_counter += 1;
            }
            Err(error) => self.pipeline_state = PipelineState::Stop(Box::new(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::step::*;
    use claims::*;

    #[tokio::test]
    async fn can_execute_step_in_pipeline() {
        let mut pipeline = StepPipeline::new();
        let mock_step_1 = MockStep::new();
        let mock_step_1_input = "tudo".to_string();
        let mock_step_2 = MockStep::new();
        let mock_step_2_input = "hello world".to_string();

        pipeline.execute_step(mock_step_1, &mock_step_1_input).await;

        pipeline.execute_step(mock_step_2, &mock_step_2_input).await;

        assert_matches!(pipeline.pipeline_state, PipelineState::Continue);
        assert_eq!(pipeline.step_counter, 2);
    }

    #[tokio::test]
    async fn can_execute_step_with_context() {
        let mut pipeline = StepPipeline::new();
        let mock_step_1 = MockStep::new();
        let mock_step_1_input = "tudo".to_string();
        let mock_step_2 = MockStep::new();
        let mock_step_2_input = "hello world".to_string();

        let mut job_context = JobContext::new();

        pipeline
            .execute_step_with_context(mock_step_1, &mock_step_1_input, &mut job_context)
            .await;

        pipeline
            .execute_step_with_context(mock_step_2, &mock_step_2_input, &mut job_context)
            .await;

        assert_matches!(pipeline.pipeline_state, PipelineState::Continue);
        assert_eq!(pipeline.step_counter, 2);
        match job_context.step_outputs.get("0").unwrap() {
            StepOutput::MockOutput(value) => {
                assert_eq!(value, &mock_step_1_input);
            }
            _ => panic!("Invalid step output"),
        };
        match job_context.step_outputs.get("1").unwrap() {
            StepOutput::MockOutput(value) => {
                assert_eq!(value, &mock_step_2_input);
            }
            _ => panic!("Invalid step output"),
        };
    }
}
