use core::{error::Error, fmt};

use error_stack::{Report, ResultExt};

#[derive(Debug)]
struct ParseExperimentError;

impl fmt::Display for ParseExperimentError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("invalid experiment description")
    }
}

impl Error for ParseExperimentError {}

fn parse_experiment(description: &str) -> Result<(u64, u64), Report<ParseExperimentError>> {
    let value = description
        .parse::<u64>()
        .attach_with(|| format!("{description:?} could not be parsed as experiment"))
        .change_context(ParseExperimentError)?;

    Ok((value, 2 * value))
}

#[derive(Debug)]
pub struct ExperimentError;

impl fmt::Display for ExperimentError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("experiment error: could not run experiment")
    }
}

impl Error for ExperimentError {}

fn start_experiments(
    experiment_ids: &[usize],
    experiment_descriptions: &[&str],
) -> Result<Vec<u64>, Report<ExperimentError>> {
    let experiments = experiment_ids
        .iter()
        .map(|exp_id| {
            let description = experiment_descriptions.get(*exp_id).ok_or_else(|| {
                Report::new(ExperimentError)
                    .attach(format!("experiment {exp_id} has no valid description"))
            })?;

            let experiment = parse_experiment(description)
                .attach(format!("experiment {exp_id} could not be parsed"))
                .change_context(ExperimentError)?;

            Ok(move || experiment.0 * experiment.1)
        })
        .collect::<Result<Vec<_>, Report<ExperimentError>>>()
        .attach("unable to set up experiments")?;

    Ok(experiments.iter().map(|experiment| experiment()).collect())
}

pub fn run() -> Result<(), Report<ExperimentError>> {
    let experiment_ids = &[0, 2];
    let experiment_descriptions = &["10", "20", "oejwofaewjifoaweijafoeijo"];

    match start_experiments(experiment_ids, experiment_descriptions) {
        Ok(results) => println!("\nExperiments ran successfully: {results:?}"),
        Err(report) => {
            eprintln!("\n{:#?}", report)
        }
    }

    Ok(())
}
