// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]

#[macro_use] extern crate boolean_enums;

gen_boolean_enum!(serde SerdeTestEnum);
gen_boolean_enum!(pub serde PubSerdeTestEnum);
gen_boolean_enum!(serde pub SerdePubTestEnum);
