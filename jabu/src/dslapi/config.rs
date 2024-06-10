use std::path::PathBuf;

use jabu_config::model::JabuProject;
use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone, CustomType)]
#[rhai_type(extra = Self::build_extra)]
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

impl ProjectConfig {
    pub fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_name("ProjectConfig")
            // Paths
            .with_fn("jaburon_file", Self::jaburon_file)
            .with_fn("target_bin", Self::target_bin)
            .with_fn("target_classes", Self::target_classes)
            .with_fn("target_docs", Self::target_docs)
            .with_fn("target_self_contained", Self::target_self_contained)
            .with_fn("lib_dir", Self::lib_dir)
            .with_fn("source_path", Self::source_path)
            .with_fn("scripts_path", Self::scripts_path)
            // Dependencies
            .with_fn("local_dependencies", Self::local_dependencies_list)
            .with_fn("remote_dependencies", Self::remote_dependencies_list);
    }

    pub fn jaburon_file(&self) -> String {
        self.directory
            .join("jabu.ron")
            .to_string_lossy()
            .to_string()
            .to_string()
    }

    pub fn target_bin(&self) -> String {
        self.directory
            .join(self.jabu_project.fs_schema.target_bin())
            .to_string_lossy()
            .to_string()
            .to_string()
    }

    pub fn target_classes(&self) -> String {
        self.directory
            .join(&self.jabu_project.fs_schema.target_classes())
            .to_string_lossy()
            .to_string()
            .to_string()
    }

    pub fn target_docs(&self) -> String {
        self.directory
            .join(self.jabu_project.fs_schema.target_docs())
            .to_string_lossy()
            .to_string()
            .to_string()
    }

    pub fn target_self_contained(&self) -> String {
        self.directory
            .join(self.jabu_project.fs_schema.target_self_contained())
            .to_string_lossy()
            .to_string()
            .to_string()
    }

    pub fn lib_dir(&self) -> String {
        self.directory
            .join(&self.jabu_project.fs_schema.lib)
            .to_string_lossy()
            .to_string()
            .to_string()
    }

    pub fn source_path(&self) -> String {
        self.directory
            .join(&self.jabu_project.fs_schema.source)
            .to_string_lossy()
            .to_string()
            .to_string()
    }

    pub fn scripts_path(&self) -> String {
        self.directory
            .join(&self.jabu_project.fs_schema.scripts)
            .to_string_lossy()
            .to_string()
            .to_string()
    }

    pub fn remote_dependencies_list(&self) -> Vec<String> {
        self.jabu_project
            .dependencies
            .remote
            .iter()
            .map(|spec| spec.to_string())
            .collect()
    }

    pub fn local_dependencies_list(&self) -> Vec<String> {
        self.jabu_project
            .dependencies
            .remote
            .iter()
            .map(|spec| spec.to_string())
            .collect()
    }
}
