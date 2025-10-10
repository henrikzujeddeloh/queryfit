use crate::config::Config;
use crate::db::Database;
use clap::{Args, Subcommand};
use linfa::dataset::DatasetBase;
use linfa::prelude::{Fit, Predict};
use linfa_linear::LinearRegression;
use ndarray::{Array1, Array2};

#[derive(Debug, Args)]
pub struct CalculateArgs {
    #[command(subcommand)]
    pub actions: Actions,
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[command(name = "rpe")]
    #[command(about = "calculate RPE for workouts without RPE")]
    RPE,
}

pub struct Workout {
    avg_hr: f64,
    avg_power: f64,
    elevation: f64,
    duration: f64,
    distance: f64,
    rpe: f64,
}

impl CalculateArgs {
    pub fn run(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        match &self.actions {
            Actions::RPE => {
                Self::run_rpe(&self, config, db)?;
            }
        }
        Ok(())
    }
    fn run_rpe(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        let workouts = Self::fetch_workouts(db)?;
        let (features, targets) = Self::prepare_data(&workouts);
        let (scaled_features, scaling_params) = Self::scale_features(&features);
        // println!("scaled_features: {:?}, scaling_params: {:?}", scaled_features, scaling_params);

        // convert to ndarray
        let features_array = Array2::from_shape_vec(
            (scaled_features.len(), scaled_features[0].len()),
            scaled_features.into_iter().flatten().collect(),
        )?;

        let targets_array = Array1::from_vec(targets);

        // Create custom dataset
        let dataset = DatasetBase::new(features_array, targets_array).with_feature_names(vec![
            "avg_hr".to_string(),
            "avg_power".to_string(),
            "elevation".to_string(),
            "duration".to_string(),
            "distance".to_string(),
        ]);

        let model = LinearRegression::new()
            .fit(&dataset)
            .expect("Failed to train model");

        let new_workout = vec![
            161.0, // avg_hr
            385.0, // avg_power
            29.0, // elevation meters
            2400.0,  // duration seconds
            7810.0, // distance meters
        ];

        // Scale new workout
        let scaled_new_workout: Vec<f64> = new_workout
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let (min_val, max_val) = scaling_params[i];
                if max_val > min_val {
                    (val - min_val) / (max_val - min_val)
                } else {
                    0.0
                }
            })
            .collect();

        // Convert scaled new workout to 2D ndarray
        let scaled_new_workout_array = Array2::from_shape_vec(
            (1, scaled_new_workout.len()),
            scaled_new_workout,
        )?;
        
        // Predict RPE
        let predicted_rpe = model.predict(&scaled_new_workout_array);
        println!("Predicted RPE: {:.2}", predicted_rpe);

        let coefficients = model.params();
        println!("Coeffs: {}", coefficients);

        // let intercept = model.intercept();
        // println!("Intercept: {}", intercept);

        Ok(())
    }

    fn fetch_workouts(db: &Database) -> anyhow::Result<Vec<Workout>> {
        let mut stmt = db.connection().prepare("SELECT sport, avg_hr, avg_power, elevation, duration, distance, rpe FROM activities WHERE sport IS 'running' AND rpe IS NOT NULL")?;

        let workout_iter = stmt.query_map([], |row| {
            Ok(Workout {
                avg_hr: row.get(1)?,
                avg_power: row.get(2)?,
                elevation: row.get(3)?,
                duration: row.get(4)?,
                distance: row.get(5)?,
                rpe: row.get(6)?,
            })
        })?;

        let workouts: Vec<Workout> = workout_iter.collect::<Result<Vec<Workout>, rusqlite::Error>>()?;
        Ok(workouts)
    }

    fn prepare_data(workouts: &[Workout]) -> (Vec<Vec<f64>>, Vec<f64>) {
        let features: Vec<Vec<f64>> = workouts
            .iter()
            .map(|w| vec![w.avg_hr, w.avg_power, w.elevation, w.duration, w.distance])
            .collect();

        let targets: Vec<f64> = workouts.iter().map(|w| w.rpe).collect();

        (features, targets)
    }

    fn scale_features(features: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<(f64, f64)>) {
        let num_features = features[0].len();

        // Calculate min and max for each feature
        let mut min_max = vec![(f64::INFINITY, f64::NEG_INFINITY); num_features];

        for sample in features {
            for (i, &val) in sample.iter().enumerate() {
                min_max[i].0 = min_max[i].0.min(val);
                min_max[i].1 = min_max[i].1.max(val);
            }
        }

        // Scale features
        let scaled_features: Vec<Vec<f64>> = features
            .iter()
            .map(|sample| {
                sample
                    .iter()
                    .enumerate()
                    .map(|(i, &val)| {
                        let (min_val, max_val) = min_max[i];
                        if max_val > min_val {
                            (val - min_val) / (max_val - min_val)
                        } else {
                            0.0
                        }
                    })
                    .collect()
            })
            .collect();

        (scaled_features, min_max)
    }
}
