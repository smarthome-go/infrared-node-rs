use infrared_rs::Scanner;

const PIN_NUMBER: u8 = 26;

fn main() {
    let mut scanner = Scanner::try_new(PIN_NUMBER).unwrap();

    loop {
        println!("{}", scanner.scan_blocking().unwrap())
    }
}
