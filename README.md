# turnip-rs

<div align="center">
    <img src="./media/turnip.png" width="25%" height="auto" alt="turnip"</img> 
</div>

Ternary operators (turnips) for Rust.

Rather than use a functional procedural macro to create yet-another domain-specific syntax, we opt for the simplest possible solution. We provide recursion without additional function calls and do not extend the Rust syntax, which does not support the `?` and `:` operators for this use case.

Our solution is a single 16-line macro defined using `macro_rules!`. [See for yourself](./src/lib.rs).

What more do you need?

## Installation

```shell
$ cargo add turnip
```

## Usage

```rust
use turnip::ifelse;

fn main() {

    result1: bool = ifelse!(10 < 0, true, false);
    result2: bool = ifelse!(10 < 0, true, 10 == 0, true, false);
    assert!(result1 == result2);

}
```

## License

MIT License
