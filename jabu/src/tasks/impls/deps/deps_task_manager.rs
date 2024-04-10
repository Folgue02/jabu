use super::{FetchDepsTask, ListDepsTask};
use crate::tasks::GeneralTaskManager;

pub fn get_deps_task_manager() -> GeneralTaskManager {
    let mut deps_taskmanager = GeneralTaskManager::default();
    deps_taskmanager.register_jabu_task("list", Box::new(ListDepsTask::default()));
    deps_taskmanager.register_jabu_task("fetch", Box::new(FetchDepsTask::default()));
    deps_taskmanager
}
