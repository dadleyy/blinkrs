## blink(rs)

[![travis.img]][travis.url] [![docs.img]][docs.url]

A rust cli tool for controlling the [blink(1)](https://blink1.thingm.com) USB LED light.

### Example

```rs
extern crate libusb;

use std::boxed::Box;
use std::error::Error;
use std::io::stdin;

use blinkrs::{Blinkers, Message};

fn main() -> Result<(), Box<dyn Error>> {
    let blinkers: Blinkers = match Blinkers::new() {
        Ok(b) => b,
        Err(_e) => {
            println!("unable to find device");
            return Ok(())
        },
    };
    blinkers.send(Message::from("red"))?;
    blinkers.send(Message::from("off"))?;
    Ok(())
}
```

## Contributing

See [CONTRIBUTING](/CONTRIBUTING.md).

[travis.img]: https://travis-ci.org/dadleyy/blinkrs.svg?branch=master
[travis.url]: https://travis-ci.org/dadleyy/blinkrs
[docs.img]: https://docs.rs/blinkrs/badge.svg
[docs.url]: https://docs.rs/blinkrs
