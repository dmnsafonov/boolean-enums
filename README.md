# boolean-enums

[![Build Status](https://travis-ci.org/dmnsafonov/boolean-enums.svg?branch=master)](https://travis-ci.org/dmnsafonov/boolean-enums)
[![Crates.io](https://img.shields.io/crates/v/boolean-enums.svg)](https://crates.io/crates/boolean-enums)
[![Documentation](https://docs.rs/boolean-enums/badge.svg)](https://docs.rs/boolean-enums)

Convenient macro to generate enums with Yes and No variants.

Useful in cases of multiple bool arguments:
```rust
#[macro_use] extern crate boolean_enums;

gen_boolean_enum!(First);
gen_boolean_enum!(Second);
gen_boolean_enum!(Third);

fn do_smth(flag1: First, flag2: Second, flag3: Third) {
    // …
}

fn main() {
    let first = First::Yes;
    let second = Second::No;
    let third = Third::Yes;

    do_smth(first, second, third); // compiles
    do_smth(first, third, second); // fails
}
```
