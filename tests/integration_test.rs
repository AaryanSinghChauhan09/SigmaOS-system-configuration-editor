// Integration test for SigmaOS-system-configuration-editor
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use system_configuration_editor as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
