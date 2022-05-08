/*!

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

use Serialize::{Log, YAML};

#[derive(Debug, YAML::Deserialize, YAML::Serialize, YAML::Partial)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    println!("{}", "Initializing ...");

    Log::import();

    let point = Point { x: 1.0, y: 2.0 };

    let string = YAML::string(&point).expect("Yaml Serial Error");

    debug!("Test Deserialization (YAML)");
    println!("{}", string);
}
```
*/

use serde;

pub use serde::{Deserialize, Serialize};

pub use serde_yaml::{Result, Value};
pub use serde_yaml::{
    mapping,
    to_string as string,
    from_str as derive,
    from_reader as file,
    to_vec as vector
};

pub use PartialEq as Partial;

pub use yaml_rust::{
    YamlLoader as Loader,
    YamlEmitter as Emitter,
    Yaml as Document
};
use serde_yaml::Mapping;

pub fn debug<Generic: std::fmt::Debug>(object: Generic) { println!("{:#?}", object) }
pub fn cast<Generic: serde::Serialize>(object: Generic) -> String {
    return format!("{:#?}", serde_yaml::to_string(&object).unwrap());
}
pub fn format<Generic: serde::Serialize>(object: Generic) -> String {
    return format!("{:}", serde_yaml::to_string(&object).unwrap());
}