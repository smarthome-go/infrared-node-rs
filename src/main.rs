use std::process;

use infrared_rs::Scanner;

mod config;
mod scanner;

const PIN_NUMBER: u8 = 26;

fn main() {
    let scanner = match Scanner::try_new(PIN_NUMBER) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not initialize scanner hardware: {e}");
            process::exit(1)
        },
    };

    if let Err(err) = scanner::start_scan(scanner) {
        eprintln!("Scanner failed unexpectedly: {err}");
        process::exit(1);
    }
}
