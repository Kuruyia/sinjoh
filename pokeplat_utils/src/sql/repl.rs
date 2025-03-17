use std::time::Instant;

use anyhow::Result;
use dialoguer::{BasicHistory, Input, console::style, theme::ColorfulTheme};
use log::{error, info};
use rusqlite::{Connection, types::ValueRef};
use tabled::settings::{Alignment, Style};

use crate::plat_loader::PlatResources;

pub struct SqlRepl {
    conn: Connection,
}

impl SqlRepl {
    pub fn from_plat_resources(resources: PlatResources) -> Result<Self> {
        let mut conn = Connection::open_in_memory()?;
        super::prepare_db_from_plat_resources(resources, &mut conn)?;

        Ok(Self { conn })
    }

    pub fn repl(&self) {
        info!("Starting SQL REPL");

        let theme = ColorfulTheme {
            prompt_prefix: style("".to_string()).for_stderr(),
            prompt_suffix: style("❯".to_string()).for_stderr(),
            ..Default::default()
        };

        let mut history = BasicHistory::new();

        loop {
            println!();
            let input = Input::<String>::with_theme(&theme)
                .history_with(&mut history)
                .interact_text();

            match input {
                Ok(input) => {
                    let handle_input_res = self.handle_input(input);

                    if let Err(e) = handle_input_res {
                        error!("Error while executing the SQL query: {}", e);
                    }
                }
                Err(e) => error!("An error has occurred while reading stdin: {}", e),
            }
        }
    }

    fn handle_input(&self, input: String) -> Result<()> {
        // Run the SQL query
        let mut stmt = self.conn.prepare(&input)?;
        let column_count = stmt.column_count();
        let column_names = stmt
            .column_names()
            .iter()
            .map(|elem| elem.to_string())
            .collect::<Vec<_>>();

        let query_start = Instant::now();
        let mut rows = stmt.query(())?;
        let query_end = Instant::now();

        // Show the results
        let mut builder = tabled::builder::Builder::default();
        builder.push_record(column_names);

        let mut results_count = 0;

        while let Some(row) = rows.next()? {
            let mut cols = Vec::new();
            results_count += 1;

            for i in 0..column_count {
                match row.get_ref(i) {
                    Ok(value) => match value {
                        ValueRef::Null => {
                            cols.push("<null>".to_string());
                        }
                        ValueRef::Integer(i) => {
                            cols.push(i.to_string());
                        }
                        ValueRef::Real(f) => {
                            cols.push(f.to_string());
                        }
                        ValueRef::Text(s) => {
                            cols.push(String::from_utf8_lossy(s).to_string());
                        }
                        ValueRef::Blob(b) => {
                            cols.push(format!("<{} bytes blob>", b.len()));
                        }
                    },
                    Err(ref _err) => {
                        cols.push("<error>".to_string());
                    }
                }
            }

            builder.push_record(cols);
        }

        let mut table = builder.build();
        table.with(Style::psql()).with(Alignment::left());

        println!("{}", table);

        // Finish input handling
        info!(
            "Got {} results in {} ms. {} row(s) affected.",
            results_count,
            (query_end - query_start).as_millis(),
            self.conn.changes()
        );

        Ok(())
    }
}
