use crate::{
    config::JabuConfig,
    tools::JavaHome,
    tasks::{JabuTask, TaskResult},
    args::parser::ParsedArguments
};
use prettytable::{Cell, Attr, Row, color};

#[derive(Default)]
pub struct ListDepsTask;

impl JabuTask for ListDepsTask {
    fn execute(&self, _: Vec<String>, _: Option<ParsedArguments>, jabu_config: &JabuConfig, _: &JavaHome) -> TaskResult {
        let dep_names: Vec<String> = jabu_config.fs_schema.get_libs().iter()
            .map(|lib| lib.file_stem().unwrap().to_str().unwrap_or("-----").to_string())
            .collect();

        Self::list_dependencies(jabu_config);
       
        Ok(())
    }


    fn description(&self) -> String {
        "List all dependencies.".to_string()
    }
}

impl ListDepsTask {
    fn list_dependencies(jabu_config: &JabuConfig) {
        let local_dependencies = &jabu_config.dependencies.local;
        let remote_dependencies = &jabu_config.dependencies.remote;
        let mut table = prettytable::Table::new();
        table.add_row(
            Row::new(
                vec![
                    Cell::new("LOCAL DEPENDENCIES")
                        .with_style(Attr::Bold)
                        //.with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
                        .with_style(Attr::ForegroundColor(color::WHITE))
                ]
            )
        );

        // --- Local dependencies list
        if local_dependencies.is_empty() {
            table.add_row(
                Row::new(
                    vec![
                        Cell::new("NO LOCAL DEPENDENCIES")
                            .with_style(Attr::Bold)
                            .with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
                            .with_style(Attr::ForegroundColor(color::RED))
                    ]
                )
            );
        } else {
            local_dependencies.iter()
                .for_each(|dep| {
                    table.add_row(
                        Row::new(vec![
                            Cell::new(&dep.artifact_name)
                                .with_style(Attr::Bold)
                                .with_style(Attr::ForegroundColor(color::BLUE)),
                            Cell::new(&dep.version)
                        ])
                    );
                });
        }

        table.add_empty_row();
        table.add_row(
            Row::new(
                vec![
                    Cell::new("REMOTE DEPENDENCIES")
                        .with_style(Attr::Bold)
                        //.with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
                        .with_style(Attr::ForegroundColor(color::WHITE))
                ]
            )
        );

        // --- Remote dependencies list
        if remote_dependencies.is_empty() {
            table.add_row(
                Row::new(
                    vec![
                        Cell::new("NO REMOTE DEPENDENCIES")
                            .with_style(Attr::Bold)
                            .with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
                            .with_style(Attr::ForegroundColor(color::RED))
                    ]
                )
            );
        } else {
            remote_dependencies.iter()
                .for_each(|dep| {
                    table.add_row(
                        Row::new(vec![
                            Cell::new(&dep.artifact_name)
                                .with_style(Attr::Bold)
                                .with_style(Attr::ForegroundColor(color::BLUE)),
                            Cell::new(&dep.version)
                        ])
                    );
                });
        }
        table.printstd();
    }
}
