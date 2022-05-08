/*!
Usage (Standard):

```no_run
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

use Serialize::{Log, JSON};

#[derive(Debug, JSON::Deserialize, JSON::Serialize, JSON::Partial)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    println!("{}", "Initializing ...");

    Log::import();

    let point = Point { x: 1.0, y: 2.0 };

    let string = JSON::string(&point).expect("JSON Serial Error");

    debug!("Test Deserialization (JSON)");
    println!("{}", string);
}

```

Usage (Pretty, Format):

```no_run
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

use Serialize::{Log, JSON};

#[derive(Debug, JSON::Deserialize, JSON::Serialize, JSON::Partial)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    println!("{}", "Initializing ...");

    Log::import();

    let point = Point { x: 1.0, y: 2.0 };

    let string = JSON::format(&point).expect("JSON Serial Error");

    debug!("Test Deserialization (JSON)");
    println!("{}", string);
}
*/

extern crate serde;

pub use serde_json as Serde;

pub use serde::{Deserialize, Serialize};

pub use serde_json::{Result, Value};
pub use serde_json::{
    map,
    to_string as string,
    to_string_pretty as pretty,
    from_str as derive,
    from_reader as file,
    self as instance,
    ser as serializer
};

pub use PartialEq as Partial;

pub fn debug<Generic: std::fmt::Debug>(object: Generic) { println!("{:#?}", object) }
pub fn cast<Generic: serde::Serialize>(object: Generic) -> String {
    return format!("{:#?}", serde_json::to_string_pretty(&object).unwrap());
}
pub fn format<Generic: serde::Serialize>(object: Generic) -> String {
    return format!("{}", serde_json::to_string_pretty(&object).unwrap());
}