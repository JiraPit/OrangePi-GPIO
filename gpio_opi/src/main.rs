use gpio_cdev::{Chip, LineRequestFlags};
use std::env;
use tokio::time;

#[tokio::main]
async fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a line number was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <line_number>", args[0]);
        std::process::exit(1);
    }

    // Parse the line number from the command-line argument
    let line_number: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid line number. Please provide a valid integer.");
            std::process::exit(1);
        }
    };

    // Open the GPIO chip
    let mut chip = match Chip::new("/dev/gpiochip0") {
        Ok(chip) => chip,
        Err(e) => {
            eprintln!("Failed to open GPIO chip: {}", e);
            std::process::exit(1);
        }
    };

    // List all lines information
    for line in chip.lines() {
        println!("{:?}", line);
    }

    // Get the specified line
    let line = match chip.get_line(line_number) {
        Ok(line) => line,
        Err(e) => {
            eprintln!("Failed to get GPIO line {}: {}", line_number, e);
            std::process::exit(1);
        }
    };

    // Request the line as input
    let handle = match line.request(LineRequestFlags::INPUT, 0, "read-input") {
        Ok(handle) => handle,
        Err(e) => {
            eprintln!("Failed to request GPIO line: {}", e);
            std::process::exit(1);
        }
    };

    // Continuously read and print the line value
    loop {
        match handle.get_value() {
            Ok(value) => println!("Value of line {}: {:?}", line_number, value),
            Err(e) => eprintln!("Failed to get line value: {}", e),
        }
        time::sleep(time::Duration::from_secs(1)).await;
    }
}
