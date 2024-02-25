use crate::{
    config::JabuConfig,
    tools::JavaHome,
    tasks::{JabuTask, TaskResult}
};
use prettytable::{Cell, Attr, Row, color::{self, Color}};

#[derive(Default)]
pub struct ListDepsTask;

impl JabuTask for ListDepsTask {
    fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> TaskResult {
        let dep_names: Vec<String> = jabu_config.fs_schema.get_libs().iter()
            .map(|lib| lib.file_stem().unwrap().to_str().unwrap_or("-----").to_string())
            .collect();

        Self::list_dependencies(dep_names, vec![]);
       
        Ok(())
    }


    fn description(&self) -> String {
        "List all dependencies.".to_string()
    }
}

impl ListDepsTask {
    fn list_dependencies(local_dependencies: Vec<String>, remote_dependencies: Vec<String>) {
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
                .for_each(|dep_name| {
                    table.add_row(
                        Row::new(vec![
                            Cell::new(dep_name)
                                .with_style(Attr::Bold)
                                .with_style(Attr::ForegroundColor(color::BLUE)),
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
                .for_each(|dep_name| {
                    table.add_row(
                        Row::new(vec![
                            Cell::new(dep_name)
                                .with_style(Attr::Bold)
                                .with_style(Attr::ForegroundColor(color::BLUE)),
                        ])
                    );
                });
        }
        table.printstd();
    }
}
