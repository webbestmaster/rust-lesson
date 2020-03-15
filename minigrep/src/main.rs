// cargo run the poem.txt
use std::process;
use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

//    let result_config = Config::new(&args);

/*
    let config = match result_config {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return ()
        },
    };
*/

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(&config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}


