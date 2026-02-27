#![allow(dead_code)]

#[derive(garde::Validate)]
#[garde(crate = "garde_as_guardian")]
struct Test<'a> {
    #[garde(ascii)]
    field: &'a str,
}

fn main() {}
