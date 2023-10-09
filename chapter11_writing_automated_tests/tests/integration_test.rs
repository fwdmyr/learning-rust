use chapter11_writing_automated_tests;

// Declare the module defined under common/mod.rs.
mod common;

// integration tests go in the tests directory. Each translation unit is compiled as its own crate.
// So this file is compiled as the "integration_test" crate.
//
// Note that files in subdirectories under the tests directory do not get compiled as test crates
// and follow the normal module rules.

#[test]
fn it_adds_stuff() {
    common::setup();
    assert_eq!(4, chapter11_writing_automated_tests::add(2, 2));
}
