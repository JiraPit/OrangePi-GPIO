use gpio_cdev::{Chip, LineRequestFlags};
use tokio::time;

#[tokio::main]
async fn main() {
    for i in 0..10 {
        let mut chip = Chip::new("/dev/gpiochip0").unwrap();
        match chip.get_line(118) {
            Ok(line) => line,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
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

    // loop {
    //     println!("Value: {:?}", handle.get_value().unwrap());
    //     time::sleep(time::Duration::from_secs(1)).await;
    // }
}
