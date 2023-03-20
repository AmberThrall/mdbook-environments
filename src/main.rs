use mdbook::config;
use mdbook_environments::*;
use std::collections::HashMap;
use std::path::PathBuf;
use clap::{arg, value_parser, Arg, ArgMatches, Command};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use semver::{Version, VersionReq};
use std::io;
use std::process;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct EnvConfig {
    template: String,
    counter_id: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub environments: HashMap<String, EnvConfig>,
}

fn read_config_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn make_app() -> Command {
    Command::new("enviornment-preprocessor")
        .arg(arg!(
            --"no-builtin" "Turn off all builtin environments"
        ))
        .arg(arg!(
                -c --config <FILE> "Load an environment config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .about("A mdbook preprocessor providing latex styled enviornments")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let mut preprocessor = EnvPreprocessor::default();
    if !matches.get_flag("no-builtin") {
        preprocessor.environments.register_builtin(BuiltinEnvironments::All);
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        let config = match read_config_file(config_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Configuration error: {}", e);
                process::exit(1);
            }
        };

        for (name, env) in config.environments.iter() {
            match preprocessor.environments.register(&name, Environment { 
                template: env.template.clone(), 
                counter_id: env.counter_id.clone(),
            }) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to register environment `{}`: {}", name, e);
                    process::exit(1);
                }
            }
        }
    }

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("Preprocessor error: {}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}