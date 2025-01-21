use gpio_cdev::{Chip, LineRequestFlags};
use tokio::time;

#[tokio::main]
async fn main() -> ! {
    // Get the chip and request a line
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let handle = chip
        .get_line(5)
        .unwrap()
        .request(LineRequestFlags::INPUT, 0, "read-input")
        .unwrap();

    // Repeatedly read the input value
    loop {
        println!("Value: {:?}", handle.get_value().unwrap());
        time::sleep(time::Duration::from_secs(1)).await;
    }
}
