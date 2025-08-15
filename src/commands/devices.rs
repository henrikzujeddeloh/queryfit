use crate::db::Database;
use crate::{config::Config, models::Device};
use chrono::DateTime;
use clap::{Args, Subcommand};
use rusqlite::params;

#[derive(Debug, Args)]
pub struct DevicesArgs {
    #[command(subcommand)]
    pub actions: Actions,
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[command(name = "list")]
    #[command(about = "list all devices")]
    List,
}

impl DevicesArgs {
    pub fn run(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        match &self.actions {
            Actions::List => self.run_list(config, db),
        }
    }

    fn run_list(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        let devices = Self::get_all_decices(db)?;

        if devices.is_empty() {
            println!("No devices found.");
            return Ok(());
        }

        println!("{:<20} {:<30} {:<10}", "Product", "Last Seen", "Battery");

        for device in devices {
            println!(
                "{:<20} {:<30} {:<10.4}V",
                device.product,
                device.timestamp.format("%Y-%m-%d %H:%M"),
                match device.battery {
                    Some(bat) => bat.to_string(),
                    None => "N/A".to_string()
                }
            );
        }

        Ok(())
    }

    fn get_all_decices(db: &Database) -> anyhow::Result<Vec<Device>> {
        let query = "SELECT * FROM devices WHERE (product, timestamp) IN (SELECT product, MAX(timestamp) FROM devices GROUP BY product) ORDER BY timestamp ASC";

        let mut stmt = db.connection().prepare(query)?;

        let devices = stmt.query_map(params![], |row| {
            let datetime_string: String = row.get(2)?;
            let parsed_datetime =
                DateTime::parse_from_rfc3339(&datetime_string).expect("Failed to parse datetime");

            Ok(Device {
                product: row.get(1)?,
                timestamp: parsed_datetime.into(),
                battery: row.get(3)?,
            })
        })?;

        let result: Vec<Device> = devices.collect::<Result<Vec<Device>, _>>()?;

        Ok(result)
    }
}
