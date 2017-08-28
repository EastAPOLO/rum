#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

extern crate userstyles;
extern crate serde;
extern crate toml;

mod add;
mod config;
mod errors {
    error_chain!{
        foreign_links {
            IoError(::std::io::Error);
            TomlError(::toml::ser::Error);
            ParseIntError(::std::num::ParseIntError);
        }
    }
}

use errors::*;
use clap::App;

quick_main!(run);
fn run() -> Result<()> {
    if !config::config_exists() {
        println!("No config file found.\n");
        config::create_config()?;
    }

    let yaml = load_yaml!("clap.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            "add" => add::run(matches.subcommand_matches("add").unwrap())?,
            "list" => unimplemented!(),
            "remove" => unimplemented!(),
            "update" => unimplemented!(),
            _ => (),
        };
    } else {
        Err("No operation specified (use -h for help)")?;
    }

    Ok(())
}
