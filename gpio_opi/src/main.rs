use gpio_cdev::{Chip, LineRequestFlags};
use tokio::time;

#[tokio::main]
async fn main() {
    for i in 0..10 {
        let mut chip = Chip::new(format!("/dev/gpiochip{i}")).unwrap();
        match chip.get_line(13) {
            Ok(line) => {
                println!("Line: {:?} connected", line);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
        match chip.get_line(7) {
            Ok(line) => {
                println!("Line: {:?} connected", line);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    }

    // loop {
    //     println!("Value: {:?}", handle.get_value().unwrap());
    //     time::sleep(time::Duration::from_secs(1)).await;
    // }
}
