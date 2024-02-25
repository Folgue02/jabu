use crate::{
    config::JabuConfig,
    tools::JavaHome,
    tasks::{GeneralTaskManager, JabuTask, TaskResult}
};

pub fn get_deps_task_manager() -> GeneralTaskManager {
    let mut deps_taskmanager = GeneralTaskManager::default();
    deps_taskmanager.register_jabu_task("list", Box::new(super::ListDepsTask::default()));
    deps_taskmanager
}
