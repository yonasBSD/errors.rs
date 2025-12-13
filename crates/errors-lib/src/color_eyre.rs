use color_eyre::{eyre::Report, eyre::WrapErr, Section};
use tracing::{info, instrument};

#[instrument]
fn main() -> Result<(), Report> {
    color_eyre::config::HookBuilder::default()
        .add_frame_filter(Box::new(|frames| {
            let filters = &["custom_filter::main"];

            frames.retain(|frame| {
                !filters.iter().any(|f| {
                    let name = if let Some(name) = frame.name.as_ref() {
                        name.as_str()
                    } else {
                        return true;
                    };

                    name.starts_with(f)
                })
            });
        }))
        .install()
        .unwrap();

    read_config()
}

#[instrument]
fn read_file(path: &str) -> Result<(), Report> {
    info!("Reading file");
    Ok(std::fs::read_to_string(path).map(drop)?)
}

#[instrument]
fn read_config() -> Result<(), Report> {
    read_file("fake_file")
        .wrap_err("Unable to read config")
        .suggestion("try using a file that exists next time")
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    //Ok(read_config()?)
    Ok(main()?)
}
