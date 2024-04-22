# rect-lib 📐

A simple library for working with anything vaguely rectangular in rust.

## Features 📦

- **Rectangle trait**: a trait implementing all rectangle operations; see [documentation](https://docs.rs/rect-lib/0.1.0/rect_lib/trait.Rectangle.html).

- **BasicRectangle**: a simple implementation of the `Rectangle` trait.

## Usage 🚀

Add the crate to your `Cargo.toml`:
```toml
[dependencies]
rect-lib = "0.1.0"
```
or use `cargo add`:
```sh
cargo add rect-lib
```

Then, you can use the `Rectangle` trait in your code:

```rust
use rect_lib::Rectangle;

#[derive(Clone, Copy)]
pub struct BasicRectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle for BasicRectangle {
    type Unit = i32;

    fn left(&self) -> i32 {
        self.x
    }

    fn right(&self) -> i32 {
        self.x + self.width - 1
    }

    fn top(&self) -> i32 {
        self.y
    }

    fn bottom(&self) -> i32 {
        self.y - self.height + 1
    }

    fn new_from_sides(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        Self {
            x: left,
            y: top,
            width: right - left + 1,
            height: top - bottom + 1,
        }
    }
}
```

## License 📜

This project is licensed under [GPL-v3](LICENSE).
