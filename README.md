# Ops

What is Ops?

It's just a joke with "try { bla bla bla } catch (NullPointerException e) { bla bla bla bla }".

But i use ops,

It's created only for pratice [declarative Rust macros](https://doc.rust-lang.org/reference/macros.html).

But it's functional macro.

Usage:

```rust
use std::fmt::Display;

use ops::catch;

fn main() {
    catch!{() =>
        try {
            error()?;
            Ok(())
        } ops e: NullPointerException {
            //do something
            println!("{}", e)
        }
    }
}

#[derive(Debug)]
struct NullPointerException {
    message: String,
}

impl NullPointerException {
    fn new(message: &str) -> Self {
        Self { message: message.into() }
    }
}

impl Display for NullPointerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NullPointerException: {}", self.message)
    }
}

impl std::error::Error for NullPointerException {}

//some stuff
fn error() -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(NullPointerException::new("get out of here")))
}

```

The macro expand to:

```rust

```