use crate::Log;

pub use clap::{
    clap_app as Macro,
    App as Application,
    Arg as Argument,
    ArgGroup as Group,
    ArgMatches as Arguments
};

pub const KEY:      &str    = "NAME";
pub const VALUE:    &str    = "ASSIGNMENT";
pub const TYPE:     &str    = "TYPE";
pub const METHOD:   &str    = "HTTP";

pub fn arguments() -> Arguments {
    let method = Argument::new(METHOD)
        .help("...")
        .long("method")
        .short("X".parse().unwrap())
        .case_insensitive(true)
        .possible_values(&[
            "GET", "POST", "DELETE", "PUT"
        ]).default_value("GET")
        .takes_value(true);

    let _type = Argument::new(TYPE)
        .help("...")
        .long("type")
        .short("t".parse().unwrap())
        .case_insensitive(true)
        .possible_values(&[
            "variable", "file"
        ]).default_value("variable")
        .takes_value(true);

    let key = Argument::new(KEY)
        .help("...")
        .long("key")
        .case_insensitive(true)
        .required(true)
        .takes_value(true);

    let value = Argument::new(VALUE)
        .help("...")
        .long("value")
        .requires(KEY)
        .required_if_eq(METHOD, "POST")
        .takes_value(true);

    let application = Application::new("CLI-Parser")
        .name("CLI-Parser")
        .bin_name("CLI-Parser")
        .version(Log::version())
        .help("GitLab VCS Repository Environment Variable Management")
            .arg(method).arg(key).arg(value);

    return application.get_matches();
}