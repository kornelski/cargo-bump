extern crate clap;
extern crate semver;
extern crate tomllib;

mod config;
mod version;

use std::path::Path;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};

use semver::Version;
use tomllib::TOMLParser;
use tomllib::types::{ParseResult, Value};

fn main() {
    let conf = config::get_config();
    let raw_data = read_file(&conf.manifest);
    let parser = TOMLParser::new();
    let (mut parser, result) = parser.parse(&raw_data);
    match result {
        ParseResult::Full => {}
        _ => panic!("couldn't parse Cargo.toml"),
    }

    let raw_value = parser
        .get_value("package.version")
        .expect("package.version missing");
    let mut version = match raw_value {
        Value::String(raw_version, _) => Version::parse(&raw_version).unwrap(),
        _ => panic!("version not a string"),
    };

    let old_version = version.clone();
    if let Some(new_version) = conf.version {
        version::update_version(&mut version, new_version);
    }

    if conf.print_version_only {
        println!("{}", version);
    } else {
        println!("Version {} -> {}", old_version, version);
    }

    parser.set_value(
        "package.version",
        Value::basic_string(version.to_string()).unwrap(),
    );

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&conf.manifest)
        .unwrap();
    f.write_all(format!("{}", parser).as_bytes()).unwrap();
}

fn read_file(file: &Path) -> String {
    let mut file = File::open(file).unwrap();
    let mut raw_data = String::new();
    file.read_to_string(&mut raw_data).unwrap();
    raw_data
}
