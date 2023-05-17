use std::collections::hash_map::Entry;
use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
use std::fs::{metadata, File};
use walkdir::WalkDir;

use mysql_cdc::binlog_reader::BinlogReader;

#[derive(Parser)]
#[command(about, version, arg_required_else_help(true))]
struct Args {
    /// Path of directory containing MySQL binlogs.
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for entry in WalkDir::new(args.path) {
        let entry = entry?;
        let path = entry.path();

        // TODO: Apply parallelism
        let f = File::open(path)?;

        // Check for an empty file
        let metadata = metadata(path)?;
        if !metadata.is_file() || metadata.len() == 0 {
            continue;
        }

        // Start parsing
        let reader = BinlogReader::new(f).unwrap();
        let mut table_map: HashMap<u64, (String, String)> = HashMap::new();

        for result in reader.read_events() {
            use mysql_cdc::events::binlog_event::BinlogEvent;
            use mysql_cdc::events::row_events::delete_rows_event::DeleteRowsEvent;
            use mysql_cdc::events::row_events::update_rows_event::UpdateRowsEvent;
            use mysql_cdc::events::row_events::write_rows_event::WriteRowsEvent;

            let (_, event) = result.unwrap();
            match event {
                BinlogEvent::UnknownEvent => continue,
                BinlogEvent::DeleteRowsEvent(DeleteRowsEvent { table_id, .. })
                | BinlogEvent::UpdateRowsEvent(UpdateRowsEvent { table_id, .. })
                | BinlogEvent::WriteRowsEvent(WriteRowsEvent { table_id, .. }) => {
                    let (database_name, table_name) = &table_map[&table_id];
                    println!("*RowsEvent\t{database_name}.{table_name}");
                }
                BinlogEvent::XidEvent(_) => continue,
                BinlogEvent::IntVarEvent(_) => continue,
                BinlogEvent::QueryEvent(event) => {
                    if event.sql_statement == "BEGIN" {
                        continue;
                    }

                    println!(
                        "QueryEvent\t\"{}\"\t\"{}\"",
                        event.database_name, event.sql_statement
                    );
                }
                BinlogEvent::TableMapEvent(event) => match table_map.entry(event.table_id) {
                    Entry::Occupied(entry) => {
                        let (database_name, table_name) = entry.get();
                        assert!(*database_name == event.database_name);
                        assert!(*table_name == event.table_name);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert((event.database_name, event.table_name));
                    }
                },
                BinlogEvent::RotateEvent(_) => continue,
                BinlogEvent::RowsQueryEvent(event) => {
                    println!("RowsQueryEvent\t\"{}\"", event.query);
                }
                BinlogEvent::HeartbeatEvent(_) => continue,
                BinlogEvent::FormatDescriptionEvent(_) => continue,
                BinlogEvent::MySqlGtidEvent(_) => continue,
                BinlogEvent::MySqlPrevGtidsEvent(_) => continue,
                BinlogEvent::MariaDbGtidEvent(_) => continue,
                BinlogEvent::MariaDbGtidListEvent(_) => continue,
            }
        }
    }

    Ok(())
}
