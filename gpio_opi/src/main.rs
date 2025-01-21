use gpio_cdev::{Chip, LineRequestFlags};
// use tokio::time;

#[tokio::main]
async fn main() {
    // List available GPIO chips
    for i in 0..5 {
        // Check first 5 possible chips
        match Chip::new(format!("/dev/gpiochip{}", i)) {
            Ok(chip) => {
                println!(
                    "Found chip {}: {} with {} lines",
                    i,
                    chip.name(),
                    chip.num_lines()
                );
            }
            Err(_) => {
                println!("No chip at gpiochip{}", i);
            }
        }
    }
}
