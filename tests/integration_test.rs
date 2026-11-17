// Integration test for SigmaOS-extended-verification-module
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use extended_verification_module as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
