use std::env;
use std::process;

use chapter12_command_line_program as minigrep;
use minigrep::Config;

fn main() {
    // Turn the iterator returned by env::args() into a vector.
    let args: Vec<String> = env::args().collect();

    // The method unwrap_or_else on Result allows for custom non-panicing error handling.
    // If Ok, unwraps Ok and assigns it.
    // Else, unwraps Err and passes its contents to the closure that that is itself passed as argument to unwrap_or_else.
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        // Nonzero error code - something went wrong.
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
