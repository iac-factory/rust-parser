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

pub use std::io::prelude::*;

pub use clap::{
    clap_app as Macro,
    App as Application,
    Arg as Argument,
    ArgGroup as Group,
    ArgMatches as Arguments
};

pub use serde::{
    Deserialize
};

pub mod Log;
pub mod YAML;
pub mod JSON;
pub mod Type;
pub mod Queue;
pub mod Stack;
pub mod HTTP;
pub mod GET;
pub mod CLI;
pub mod Constants;
