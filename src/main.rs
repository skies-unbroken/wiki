extern crate clap;

use clap::{load_yaml, App};
use wiki::wiki::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();
    if let Some(subcommand) = matches.subcommand_name() {
        let matches = matches.subcommand_matches(subcommand).unwrap(); //intentionally shadowing
        match subcommand {
            "search" => {
                if let Some(keywords) = matches.value_of("keywords") {
                    let request = Request::new(Query::Search, keywords);
                    let response = request.search().unwrap();
                    match matches.is_present("snippet") {
                        true => for (result, snippet) in response { println!("{}: {}\n", result, snippet) }
                        false => for (result, _) in response { println!("{}\n", result) }
                    }
                } else { panic!("You must enter some keywords.") }
            }
            _ => unimplemented!()
        };
    } else {
        unimplemented!()
    }
}