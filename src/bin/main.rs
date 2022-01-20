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
  Ok(Message::Fade(
    Color::Three(red, green, blue),
    Duration::new(secs, 0),
    None,
  ))
}

fn zip<E, T, U>(first: Result<T, E>, second: Result<U, E>) -> Result<(T, U), E> {
  first.and_then(|a| second.map(|b| (a, b)))
}

fn parse<S>(input: S) -> Option<Message>
where
  S: AsRef<str>,
{
  let bits = input.as_ref().split(" ").collect::<Vec<&str>>();

  match bits[..] {
    ["i", i, f, r, g, b] => {
      let index = i.parse::<u8>();
      let fade = f.parse::<u64>();
      let red = r.parse::<u8>();
      let green = g.parse::<u8>();
      let blue = b.parse::<u8>();
      let color = zip(zip(red, green), blue).map(|((r, g), b)| Color::Three(r, g, b));
      let message = zip(zip(fade, index), color)
        .map(|((fade, index), color)| Message::Fade(color, Duration::from_millis(fade), Some(index)));
      message.ok()
    }

    ["i", i, r, g, b] => {
      let index = i.parse::<u8>();
      let red = r.parse::<u8>();
      let green = g.parse::<u8>();
      let blue = b.parse::<u8>();
      let color = zip(zip(red, green), blue).map(|((r, g), b)| Color::Three(r, g, b));
      let message = zip(index, color).map(|(index, color)| Message::Immediate(color, Some(index)));
      message.ok()
    }
    [one, red_str, green_str, blue_str] => parse_bits((one, red_str, green_str, blue_str)).ok(),
    [one, two] => {
      let dur = one.parse::<u64>().map(Duration::from_secs).ok()?;
      Some(Message::Fade(Color::from(two), dur, None))
    }
    _ => Some(Message::from(input.as_ref())),
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let blinkers: Blinkers = Blinkers::new()?;

  println!("found {:?} devices", blinkers.device_count());

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

    let msg = parse(&trimmed).unwrap_or_else(|| {
      println!("[warning] no message for '{}', sending OFF", trimmed);
      Message::default()
    });

    if let Err(e) = blinkers.send(msg) {
      println!("[err] unable to send: {:?}", e);
      continue;
    }

    println!("[ok] input: {:?}, sent {:?}", trimmed, msg);
  }
}
