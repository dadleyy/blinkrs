use std::boxed::Box;
use std::error::Error;
use std::io::stdin;
use std::time::Duration;

use blinkrs::{Blinkers, Color, Message};

fn parse_bits(bits: (&str, &str, &str, &str)) -> Result<Message, std::num::ParseIntError> {
  let secs = bits.0.parse::<u64>()?;
  let red = bits.1.parse::<u8>()?;
  let green = bits.2.parse::<u8>()?;
  let blue = bits.3.parse::<u8>()?;
  Ok(Message::Fade(Color::Three(red, green, blue), Duration::new(secs, 0)))
}

fn main() -> Result<(), Box<dyn Error>> {
  let blinkers: Blinkers = Blinkers::new()?;

  loop {
    let mut message = String::new();
    println!("input command (red|blue|green|exit):");

    stdin().read_line(&mut message)?;
    let trimmed = message.trim();

    if trimmed == String::from("exit") {
      println!("[ok] received exit command");

      if let Err(e) = blinkers.send(Message::Off) {
        println!("[error] unable to send off mesage: {:?}", e);
      }

      return Ok(());
    }

    let bits = trimmed.split(" ").collect::<Vec<&str>>();

    let msg = match bits[..] {
      [one, red_str, green_str, blue_str] => match parse_bits((one, red_str, green_str, blue_str)) {
        Ok(m) => m,
        Err(e) => {
          println!("[error] unable to parse bits: {:?}", e);
          continue;
        }
      },
      [one, two] => {
        let dur = match one.parse::<u64>() {
          Ok(t) => Duration::new(t, 0),
          Err(e) => {
            println!("[err] unable to parse time: {}", e);
            continue;
          }
        };
        println!("[debug] found fade {} for color {}", one, two);
        Message::Fade(Color::from(two), dur)
      }
      _ => Message::from(trimmed),
    };

    if let Err(e) = blinkers.send(msg) {
      println!("[err] unable to send: {:?}", e);
      continue;
    }

    println!("[ok] input: {:?}, sent {:?}", trimmed, msg);
  }
}
