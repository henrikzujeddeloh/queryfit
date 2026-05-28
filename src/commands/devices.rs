use crate::db::Database;
use crate::{config::Config, models::Device};
use chrono::{DateTime, Local};
use clap::{Args, Subcommand};
use rusqlite::types::Type;
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

    fn run_list(&self, _config: &Config, db: &Database) -> anyhow::Result<()> {
        let devices = Self::get_all_devices(db)?;

        if devices.is_empty() {
            println!("No devices found.");
            return Ok(());
        }

        println!(
            "{:<20} {:<12} {:<10} {:<10} {}",
            "Product", "Last Seen", "Battery", "Status", "Timestamp"
        );

        for device in devices {
            let battery = Self::format_battery(device.battery);
            let status = Self::format_battery_status(device.battery_status.as_deref());
            println!(
                "{:<20} {:<12} {:<10} {:<10} {}",
                device.product,
                Self::format_last_seen(device.timestamp),
                battery,
                status,
                device.timestamp.format("%Y-%m-%d %H:%M"),
            );
        }

        Ok(())
    }

    fn get_all_devices(db: &Database) -> anyhow::Result<Vec<Device>> {
        let query = "
            WITH latest_seen AS (
                SELECT product, MAX(timestamp) AS timestamp
                FROM devices
                GROUP BY product
            )
            SELECT
                latest_seen.product,
                latest_seen.timestamp,
                (
                    SELECT devices.battery
                    FROM devices
                    WHERE devices.product = latest_seen.product
                      AND devices.battery IS NOT NULL
                    ORDER BY devices.timestamp DESC, devices.rowid DESC
                    LIMIT 1
                ) AS battery,
                (
                    SELECT devices.battery_status
                    FROM devices
                    WHERE devices.product = latest_seen.product
                      AND devices.battery_status IS NOT NULL
                      AND devices.battery_status != ''
                    ORDER BY devices.timestamp DESC, devices.rowid DESC
                    LIMIT 1
                ) AS battery_status
            FROM latest_seen
            ORDER BY latest_seen.timestamp DESC, latest_seen.product ASC
        ";

        let mut stmt = db.connection().prepare(query)?;

        let devices = stmt.query_map(params![], |row| {
            let datetime_string: String = row.get(1)?;
            let parsed_datetime = DateTime::parse_from_rfc3339(&datetime_string).map_err(|err| {
                rusqlite::Error::FromSqlConversionFailure(1, Type::Text, Box::new(err))
            })?;

            Ok(Device {
                product: row.get(0)?,
                timestamp: parsed_datetime.into(),
                battery: row.get(2)?,
                battery_status: row.get(3)?,
            })
        })?;

        let result: Vec<Device> = devices.collect::<Result<Vec<Device>, _>>()?;

        Ok(result)
    }

    fn format_battery(battery: Option<f64>) -> String {
        match battery {
            Some(voltage) => format!("{voltage:.2} V"),
            None => "N/A".to_string(),
        }
    }

    fn format_battery_status(status: Option<&str>) -> &str {
        status.filter(|value| !value.is_empty()).unwrap_or("unknown")
    }

    fn format_last_seen(timestamp: DateTime<Local>) -> String {
        let duration = Local::now().signed_duration_since(timestamp);

        if duration.num_days() >= 1 {
            format!("{}d ago", duration.num_days())
        } else if duration.num_hours() >= 1 {
            format!("{}h ago", duration.num_hours())
        } else if duration.num_minutes() >= 1 {
            format!("{}m ago", duration.num_minutes())
        } else {
            "just now".to_string()
        }
    }
}
