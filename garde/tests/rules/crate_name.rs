use garde as garde_as_guardian;

use super::util;

#[derive(Debug, garde_as_guardian::Validate)]
#[garde(crate = "garde_as_guardian")]
struct Test<'a> {
    #[garde(ascii)]
    field: &'a str,
}

#[derive(Debug, garde_as_guardian::Validate)]
#[garde(crate = "garde_as_guardian")]
struct TestTuple<'a>(#[garde(ascii)] &'a str);

#[derive(Debug, garde_as_guardian::Validate)]
#[garde(crate = "garde_as_guardian")]
struct TestNested<'a> {
    #[garde(dive)]
    test: Test<'a>,
    #[garde(dive)]
    tuple: TestTuple<'a>,
}

#[test]
fn ascii_valid() {
    util::check_ok(&[Test { field: "a!0_~" }], &());
    util::check_ok(&[TestTuple("a!0_~")], &());
    util::check_ok(
        &[TestNested {
            test: Test { field: "" },
            tuple: TestTuple(""),
        }],
        &(),
    );
}

#[test]
fn ascii_invalid() {
    util::check_fail!(
        &[TestNested {
            test: Test { field: "😂" },
            tuple: TestTuple("😂"),
        }],
        &(),
    );
}
