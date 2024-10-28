# 5.0.0
This edition is technically breaking since it removes the `std` feature and implements `Error` from `core` instead of `std`.

- Update registry to latest version.
- Implement `Error` from `core` instead of `std`.
- Remove feature `std`.
- Update edition.
- Update documentation links.

# 4.0.1
This version generated country-specific code automatically from the IBAN registry. It also tests all examples that can be tested. The registry is somewhat messy and inconsistent actually, and as such still requires quite some manual effort. Sometimes the registry is plainly incorrect and national documentation was used instead. This means that the crate now performs better than some of the online tools I've tested.

- Fix some country-specific code.
- Test for compliance with all applicable examples in the registry.
- Update to the newest IBAN registry (October 2021).
- Drop minimal supported version. (The minimal supported version was already increased by dependencies, so from now on just the latest stable is guaranteed.)

# 4.0.0

This version overhauls the internal parsing code entirely. It removes its dependency on `regex`, which allows the crate to be run in `no_std` mode. It also speeds up parsing by a factor of about 100. The implemented changes:

- _Breaking_: Added a feature (enabled by default) `std`, which enables usage of the standard library. Other than the lack of `std::error::Error`, there are no restrictions on `no_std` mode.
- Follow the standard more closely: allow lowercase characters in the BBAN position but normalize them, and disallow `00` or `01` as check characters.
- Updated to follow latest IBAN spec.
- Added sub crate in workspace for fuzzing with AFL.
- Many functions are a lot faster.
