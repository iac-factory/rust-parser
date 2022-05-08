#![allow(unreachable_code)]

#![recursion_limit = "4096"]

use crate::HTTP;
use crate::HTTP::{Body, Writer, Reader, URL};

use log::{*};

use std::io::Read;
use std::fs::File;
use std::str::FromStr;

use hyper::{Response, StatusCode};
use hyper::Request;

use serde::Serialize;
use serde::Deserialize;

use serde_json::{Result as Serial, Value};

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Variable {
    variable_type: String,
    key: String,
    value: String,
    protected: bool,
    masked: bool,
    environment_scope: String
}

#[HTTP::Asynchronous()]
pub async fn post(typ: &str, key: &str, value: &str, protected: bool, masked: bool, scope: &str) -> Result<(), HTTP::Heap> {
    let mut log = tokio::fs::File::create(".log").await.expect("Error Creating Log File");

    let mut client = HTTP::Client::builder()
        .build::<(_), HTTP::Data>(HTTP::TLS::new());

    let variable: Variable = Variable {
        variable_type: format!("{}", typ),
        key: format!("{}", key),
        value: format!("{}", value),
        environment_scope: format!("{}", scope),
        protected, masked
    };

    let serial: String = serde_json::to_string_pretty(&variable)?;

    debug!("Body: {:#?}", serial);

    let mut host = url::Url::parse("https://<gitlab.example.com>/api/v4/projects/89/variables")?;

    host.query_pairs_mut().append_pair("value", value);
    host.query_pairs_mut().append_pair("key", key);
    host.query_pairs_mut().append_pair("variable_type", typ);
    host.query_pairs_mut().append_pair("environment_scope", scope);
    host.query_pairs_mut().append_pair("masked", &masked.to_string());
    host.query_pairs_mut().append_pair("protected", &protected.to_string());

    debug!("{}", host);

    let request = Request::builder()
        .method(hyper::Method::POST)
        .uri(host.as_str())
        .header("PRIVATE-TOKEN", "<token>")
        .header("Content-Type", "application/json")
        .body(hyper::Body::empty())
            .unwrap();

    let mut response = client.request(request).await?;

    while let Some(chunk) = response.body_mut().data().await {
        log.write(&chunk.expect("")).await.expect(
            "Error Writing to Buffer"
        );
    }

    let mut file = File::open(".log").expect("Error Opening File Reader");

    let mut buffer = String::new();

    let _ = file.read_to_string(&mut buffer)?;

    let success = response.status().is_success();

    if !success {
        error!("Error: Status {:#?}", response.status());
        error!("Error: Headers {:#?}", response.headers());
        error!("Error: HTTP {:#?}", response.version());
        error!("Error: Message {:#?}", &buffer.as_str());
    } else {
        info!("{}", &buffer);
    }

    return Ok(());
}


#[HTTP::Asynchronous()]
pub async fn main() -> Result<(), HTTP::Heap> {
    let mut log = tokio::fs::File::create(".log").await.expect("Error Creating Log File");

    let mut client = HTTP::Client::builder()
        .build::<(_), HTTP::Data>(HTTP::TLS::new());

    let request = Request::builder()
        .method(hyper::Method::GET)
        .uri("https://<gitlab.example.com>/api/v4/projects/89/variables")
        .header("PRIVATE-TOKEN", "<token>")
        .body(hyper::Body::default()
    ).unwrap();

    let mut response = client.request(request).await?;

    while let Some(chunk) = response.body_mut().data().await {
        log.write(&chunk.expect("")).await.expect(
            "Error Writing to Buffer"
        );
    }

    let mut file = File::open(".log").expect("Error Opening File Reader");

    let mut buffer = String::new();

    let _ = file.read_to_string(&mut buffer)?;

    info!("{}", &buffer);

    return Ok(());
}
