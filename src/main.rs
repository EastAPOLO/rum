#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate reqwest;
extern crate toml;
extern crate userstyles;

mod add;
mod list;
mod remove;
mod config;
mod update;
mod userstyle;
mod errors {
    error_chain!{
        foreign_links {
            IoError(::std::io::Error);
            TomlError(::toml::de::Error);
            ReqwestError(::reqwest::Error);
            TomlSerError(::toml::ser::Error);
            ParseIntError(::std::num::ParseIntError);
        }
    }
}

use errors::*;
use clap::App;

quick_main!(run);
fn run() -> Result<()> {
    if !config::config_exists() {
        eprintln!("\x1b[0;31;40mNo config file found.\x1b[0m\n");
        config::create_config()?;
    }

    let yaml = load_yaml!("clap.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            "add" => add::run(matches.subcommand_matches("add").unwrap())?,
            "list" => list::run(matches.subcommand_matches("list").unwrap())?,
            "remove" => remove::run(matches.subcommand_matches("remove").unwrap())?,
            "update" => update::run(matches.subcommand_matches("update").unwrap())?,
            _ => (),
        };
    } else {
        Err("No operation specified (use -h for help)")?;
    }

    Ok(())
}
