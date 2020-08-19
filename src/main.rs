#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

extern crate clap;
extern crate regex;

use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.value_of("language").unwrap() {
        "cpp" => 
        {
            println!("Setting up language=C++");
        },
        
        "c" => 
        {
            println!("Setting up language=C");
        },

        "rust" =>
        {
            println!("Setting up language=Rust");
        },

        "python" => 
        {
            println!("Setting up language=Python");
        },
        
        _ => unreachable!("No language have been selected!"),
    };

    match matches.value_of("licen").unwrap() {
        "GPLv3" =>
        {
            println!("Adding License=GPLv3");
        },

        "AGPLv3" =>
        {
            println!("Adding License=AGPLv3");
        },

        "LGPLv3" =>
        {
            println!("Adding License=LGPLv3");
        },

        "MOZILLA-PUBLIC-LICENSE-2" =>
        {
            println!("Adding License=MOZILLA-PUBLIC-LICENSE-2");
        },

        "MIT" =>
        {
            println!("Adding License=MIT");
        },

        "APACHE-2" =>
        {
            println!("Adding License=APACHE-2");
        },
 
        _ => unreachable!("No License have been selected!"),

    };
}
