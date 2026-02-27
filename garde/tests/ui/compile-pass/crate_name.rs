#![allow(dead_code)]

use garde as garde_as_guardian;

#[derive(garde_as_guardian::Validate)]
#[garde(crate = "garde_as_guardian")]
struct Test<'a> {
    #[garde(ascii)]
    field: &'a str,
}

fn main() {}
