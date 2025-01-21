use gpio_cdev::{Chip, LineRequestFlags};
use tokio::time;

#[tokio::main]
async fn main() {
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let line = chip.get_line(13).unwrap();
    let handle = line
        .request(LineRequestFlags::INPUT, 0, "read-input")
        .unwrap();
    loop {
        println!("Value: {:?}", handle.get_value().unwrap());
        time::sleep(time::Duration::from_secs(1)).await;
    }
}
