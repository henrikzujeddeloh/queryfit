use crate::config::Config;
use crate::db::Database;
use clap::{Args, Subcommand};
use rusqlite::params;

#[derive(Debug, Args)]
pub struct SummaryArgs {
    #[command(subcommand)]
    pub actions: Actions,
}

#[derive(Debug, Args)]
pub struct SummarySubcommandArgs {
    #[arg(long)]
    pub activity: Option<Vec<String>>,
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[command(name = "7d")]
    #[command(about = "summarize statistics over the last 7 days")]
    SevenDays(SummarySubcommandArgs),

    #[command(name = "30d")]
    #[command(about = "summarize statistics over the last 30 days")]
    ThirtyDays(SummarySubcommandArgs),

    #[command(name = "365d")]
    #[command(about = "summarize statistics over the last 365 days")]
    ThreeSixFiveDays(SummarySubcommandArgs),
}

impl SummaryArgs {
    pub fn run(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        match &self.actions {
            Actions::SevenDays(args) => {
                self.run_seven_days(config, db, args)?;
            }
            Actions::ThirtyDays(args) => {
                self.run_thirty_days(config, db, args)?;
            }
            Actions::ThreeSixFiveDays(args) => {
                self.run_threesixfive_days(config, db, args)?;
            }
        }
        Ok(())
    }

    pub fn run_seven_days(
        &self,
        config: &Config,
        db: &Database,
        args: &SummarySubcommandArgs,
    ) -> anyhow::Result<()> {
        match &args.activity {
            Some(activity) => {
                println!("7-Day Summary for {:?}\n", activity);
            }
            None => println!("7-Day Summary\n"),
        }

        println!(
            "Total Duration: {}",
            Self::format_duration(Self::sum_duration_last_n_days(db, 7, args)?)
        );
        println!(
            "Total Distance: {:.2} km",
            Self::sum_distance_last_n_days(db, 7, args)? / 1000.0
        );
        println!(
            "Average Calories: {:.2} kcal",
            Self::avg_calories_last_n_days(db, 7, args)?
        );

        Ok(())
    }

    pub fn run_thirty_days(
        &self,
        config: &Config,
        db: &Database,
        args: &SummarySubcommandArgs,
    ) -> anyhow::Result<()> {
        match &args.activity {
            Some(activity) => {
                println!("30-Day Summary for {:?}\n", activity);
            }
            None => println!("30-Day Summary\n"),
        }

        println!(
            "Total Duration: {}",
            Self::format_duration(Self::sum_duration_last_n_days(db, 30, args)?)
        );
        println!(
            "Total Distance: {:.2} km",
            Self::sum_distance_last_n_days(db, 30, args)? / 1000.0
        );
        println!(
            "Average Calories: {:.2} kcal",
            Self::avg_calories_last_n_days(db, 30, args)?
        );

        Ok(())
    }

    pub fn run_threesixfive_days(
        &self,
        config: &Config,
        db: &Database,
        args: &SummarySubcommandArgs,
    ) -> anyhow::Result<()> {
        match &args.activity {
            Some(activity) => {
                println!("365-Day Summary for {:?}\n", activity);
            }
            None => println!("365-Day Summary\n"),
        }

        println!(
            "Total Duration: {}",
            Self::format_duration(Self::sum_duration_last_n_days(db, 365, args)?)
        );
        println!(
            "Total Distance: {:.2} km",
            Self::sum_distance_last_n_days(db, 365, args)? / 1000.0
        );
        println!(
            "Average Calories: {:.2} kal",
            Self::avg_calories_last_n_days(db, 365, args)?
        );

        Ok(())
    }

    fn sum_distance_last_n_days(
        db: &Database,
        days: u16,
        args: &SummarySubcommandArgs,
    ) -> anyhow::Result<f64> {
        let base_query = "SELECT SUM(distance) FROM activities WHERE distance IS NOT NULL AND timestamp >= datetime('now', ?)";

        let query = if let Some(activity_list) = &args.activity {
            let activities_condition = activity_list
                .iter()
                .map(|a| format!("'{}'", a))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{} AND sport IN ({})", base_query, activities_condition)
        } else {
            base_query.to_string()
        };

        let sum_distance: Option<f64> =
            db.connection()
                .query_row(&query, params![format!("-{} days", days)], |row| row.get(0))?;

        Ok(sum_distance.unwrap_or(0.0))

        // let sum_distance: f64 = db.connection().query_row("SELECT SUM(distance) FROM activities WHERE distance IS NOT NULL AND timestamp >= datetime('now', ?)", params![format!("-{} days", days)], |row| row.get(0))?;
        // Ok(sum_distance)
    }

    fn sum_duration_last_n_days(
        db: &Database,
        days: u16,
        args: &SummarySubcommandArgs,
    ) -> anyhow::Result<f64> {
        let base_query =
            "SELECT SUM(duration) FROM activities WHERE timestamp >= datetime('now', ?)";

        let query = if let Some(activity_list) = &args.activity {
            let activities_condition = activity_list
                .iter()
                .map(|a| format!("'{}'", a))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{} AND sport IN ({})", base_query, activities_condition)
        } else {
            base_query.to_string()
        };

        let sum_duration: Option<f64> =
            db.connection()
                .query_row(&query, params![format!("-{} days", days)], |row| row.get(0))?;

        Ok(sum_duration.unwrap_or(0.0))

        // let sum_distance: f64 = db.connection().query_row("SELECT SUM(distance) FROM activities WHERE distance IS NOT NULL AND timestamp >= datetime('now', ?)", params![format!("-{} days", days)], |row| row.get(0))?;
        // Ok(sum_distance)
    }

    fn avg_calories_last_n_days(
        db: &Database,
        days: u16,
        args: &SummarySubcommandArgs,
    ) -> anyhow::Result<f64> {
        let base_query =
            "SELECT AVG(calories) FROM activities WHERE timestamp >= datetime('now', ?)";

        let query = if let Some(activity_list) = &args.activity {
            let activities_condition = activity_list
                .iter()
                .map(|a| format!("'{}'", a))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{} AND sport IN ({})", base_query, activities_condition)
        } else {
            base_query.to_string()
        };

        let avg_calories: Option<f64> =
            db.connection()
                .query_row(&query, params![format!("-{} days", days)], |row| row.get(0))?;

        Ok(avg_calories.unwrap_or(0.0))

        // let sum_distance: f64 = db.connection().query_row("SELECT SUM(distance) FROM activities WHERE distance IS NOT NULL AND timestamp >= datetime('now', ?)", params![format!("-{} days", days)], |row| row.get(0))?;
        // Ok(sum_distance)
    }

    fn format_duration(seconds: f64) -> String {
        let hours = (seconds / 3600.0).floor() as u64;
        let remaining_seconds = seconds % 3600.0;
        let mins = (remaining_seconds / 60.0).floor() as u64;
        let secs = (remaining_seconds % 60.0).floor() as u64;

        format!("{:02} hours {:02} minutes {:02} seconds", hours, mins, secs)
    }
}
