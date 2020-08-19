#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

extern crate clap;
extern crate regex;

use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.value_of("lang").unwrap() {
        "none" => println!("No Language have been selected!"),
        
        "cpp" => 
        {
            println!("Hello");
        },
        
        "c" => 
        {
            println!("مرحبا");
        },

        "rust" =>
        {
            println!("Hallo");
        },

        "python" => 
        {
            println!("Pythos goes brrr....");
        },
        
        _ => unreachable!("see possible_values in yaml, handled by clap"),
    };
}
