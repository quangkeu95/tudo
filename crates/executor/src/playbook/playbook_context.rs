use std::sync::Arc;

use derive_builder::Builder;
use futures::lock::Mutex;
use interpreter::playbook::Setup;

/// Playbook context, could be shared with the whole playbook by using [`SharedMutexPlaybookContext`]
#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct PlaybookContext {
    #[builder(default)]
    shared_setup: Option<Arc<Setup>>,
}

pub type SharedMutexPlaybookContext = Arc<Mutex<PlaybookContext>>;

impl PlaybookContext {
    pub fn into_shared_mutex(self) -> SharedMutexPlaybookContext {
        Arc::new(Mutex::new(self))
    }
}
