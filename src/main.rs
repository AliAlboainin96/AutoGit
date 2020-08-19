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

    match matches.value_of("License").unwrap() { 
    "
    };
}
