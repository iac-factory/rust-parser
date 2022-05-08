#![allow(warnings)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unsafe_code)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(rust_2018_idioms)]
#![allow(unreachable_code)]

#![recursion_limit = "4096"]

#[macro_use] extern crate log;

use std::env;

use Parser::{Log, YAML, JSON, CLI, GET};
use Parser::HTTP::{HTTP, HTTPs, Client,};

fn main() -> Result<(), ()> {
    Log::import();

    let user = env::home_dir().expect(
        "Unable to Read Home Directory"
    );

    debug!("{}", "Initializing ...");
    //
    // GET::main();

    let arguments = CLI::arguments();

    let variable = arguments.value_of(CLI::KEY).unwrap();
    let assignment = arguments.value_of(CLI::VALUE)
        .unwrap_or_default();

    let typ = match arguments.value_of(CLI::TYPE) {
        Some(_) => "env_var",
        None => "env_var"
    };

    info!("Variable: {}", variable);
    info!("Assignment: {}", assignment);

    GET::post(typ, variable, assignment, true, true, "*");

    // if let Some(iterator) = arguments.values_of("EXCLUSION") {
    //     for value in iterator {
    //         exclusions.push(value);
    //     }
    //
    //     let exclusion = String::from("Exclusions.json");
    //
    //     let mut exclude = std::fs::File::create(&&exclusion)?;
    //
    //     exclude.write_all(Parser::JSON::format(&exclusions).as_ref());
    //
    //     debug!("{}", "Successfully Wrote Exclusions.json File");
    // }

    debug!("{}", "Complete");

    return Ok(());
}
