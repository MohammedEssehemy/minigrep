use std::env;
use std::process;

use minigrep_essehemy::Config;
use minigrep_essehemy::run;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::from(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });


    let config = Config::from(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}