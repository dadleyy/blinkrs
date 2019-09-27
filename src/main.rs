extern crate libusb;

use std::boxed::Box;
use std::error::Error;
use std::io::stdin;

use blinkrs::message::Message;
use blinkrs::Blinkers;

fn main() -> Result<(), Box<dyn Error>> {
    let blinkers: Blinkers = Blinkers::new()?;

    loop {
        let mut message = String::new();
        println!("input command (red|blue|green|exit):");

        stdin().read_line(&mut message)?;
        let trimmed = message.trim();

        if trimmed == String::from("exit") {
            println!("[ok] received exit command");
            return Ok(());
        }

        println!("[ok] input: {:?}", trimmed);
        blinkers.send(Message::from(trimmed))?;
    }
}
