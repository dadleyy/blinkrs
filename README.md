## blink(rs)

[![ci.img]][ci.url] [![docs.img]][docs.url]

A rust cli tool for controlling the [blink(1)](https://blink1.thingm.com) USB LED light.

### Example

```rust
use std::boxed::Box;
use std::error::Error;

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

[ci.img]: https://github.com/dadleyy/blinkrs/workflows/gh.build-and-test/badge.svg?flat
[ci.url]: https://github.com/dadleyy/blinkrs/actions?workflow=gh.build-and-test
[docs.img]: https://docs.rs/blinkrs/badge.svg
[docs.url]: https://docs.rs/blinkrs
