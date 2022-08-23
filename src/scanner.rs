pub fn start_scan(device: infrared_rs::Scanner) -> Result<(), infrared_rs::Error> {
    loop {
        let code = device.scan_blocking()?;
        println!("Scanned code: {:x}", code)
    }
}
