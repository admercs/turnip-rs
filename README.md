# turnip-rs

<div align="center">
    <img src="./media/turnip.png" width="25%" height="auto" alt="turnip"</img> 
</div>

Ternary operators (turnips) for Rust.

Rather than creating a functional procedural macro to parse yet-another domain-specific syntax, I opt for the simplest possible solution. The chosen approach provides recursion without additional function calls and does not extend the Rust syntax, which does not support the `?` and `:` operators for this use case.

The solution is a single 16-line macro defined using `macro_rules!`. [See for yourself](./src/lib.rs). Unlike other solutions out there, such as [terny](https://github.com/KaitlynEthylia/terny), [tern](https://github.com/lmburns/tern), [iffy](https://github.com/zfzackfrost/iffy-rs) and [ternop](https://github.com/spacekookie/ternop.rs), this one provides recursion without additional top-level function calls, more closely matching the ternary operator design pattern.

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
