use std::path::PathBuf;

use jabu_config::model::JabuProject;
use pyo3::prelude::*;

#[pyclass]
pub struct ProjectConfig {
    jabu_project: JabuProject,
    directory: PathBuf,
}

impl ProjectConfig {
    pub fn new(jabu_project: JabuProject, directory: PathBuf) -> Self {
        Self {
            jabu_project,
            directory,
        }
    }
}

impl From<JabuProject> for ProjectConfig {
    fn from(value: JabuProject) -> Self {
        Self::new(value, std::env::current_dir().unwrap())
    }
}

#[pymethods]
impl ProjectConfig {
    pub fn jaburon_file(&self) -> PyResult<String> {
        Ok(self
            .directory
            .join("jabu.ron")
            .to_string_lossy()
            .to_string()
            .to_string())
    }

    pub fn target_bin(&self) -> PyResult<String> {
        Ok(self
            .directory
            .join(self.jabu_project.fs_schema.target_bin())
            .to_string_lossy()
            .to_string()
            .to_string())
    }

    pub fn target_classes(&self) -> PyResult<String> {
        Ok(self
            .directory
            .join(&self.jabu_project.fs_schema.target_classes())
            .to_string_lossy()
            .to_string()
            .to_string())
    }

    pub fn target_docs(&self) -> PyResult<String> {
        Ok(self
            .directory
            .join(self.jabu_project.fs_schema.target_docs())
            .to_string_lossy()
            .to_string()
            .to_string())
    }

    pub fn target_self_contained(&self) -> PyResult<String> {
        Ok(self
            .directory
            .join(self.jabu_project.fs_schema.target_self_contained())
            .to_string_lossy()
            .to_string()
            .to_string())
    }

    pub fn lib_dir(&self) -> PyResult<String> {
        Ok(self
            .directory
            .join(&self.jabu_project.fs_schema.lib)
            .to_string_lossy()
            .to_string()
            .to_string())
    }

    pub fn source_path(&self) -> PyResult<String> {
        Ok(self
            .directory
            .join(&self.jabu_project.fs_schema.source)
            .to_string_lossy()
            .to_string()
            .to_string())
    }

    pub fn scripts_path(&self) -> PyResult<String> {
        Ok(self
            .directory
            .join(&self.jabu_project.fs_schema.scripts)
            .to_string_lossy()
            .to_string()
            .to_string())
    }

    pub fn remote_dependencies_list(&self) -> PyResult<Vec<String>> {
        Ok(self
            .jabu_project
            .dependencies
            .remote
            .iter()
            .map(|spec| spec.to_string())
            .collect())
    }

    pub fn local_dependencies_list(&self) -> PyResult<Vec<String>> {
        Ok(self
            .jabu_project
            .dependencies
            .remote
            .iter()
            .map(|spec| spec.to_string())
            .collect())
    }
}
