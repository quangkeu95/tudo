use thiserror::Error;
use tudo_interpreter::playbook::Playbook;

pub struct Executor {}

impl Executor {
    pub fn run(playbook: &Playbook) -> Result<(), ExecutorError> {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ExecutorError {}
