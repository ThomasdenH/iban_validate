# 4.0.0

This version overhauls the internal parsing code entirely. It removes its dependency on `regex`, which allows the crate to be run in `no_std` mode. It also speeds up parsing by a factor of about 100. The implemented changes:

- _Breaking_: Added a feature (enabled by default) `std`, which enables usage of the standard library. Other than the lack of `std::error::Error`, there are no restrictions on `no_std` mode.
- Follow the standard more closely: allow lowercase characters in the BBAN position but normalize them, and disallow `00` or `01` as check characters.
- Updated to follow latest IBAN spec.
- Added sub crate in workspace for fuzzing with AFL.
- Many functions are a lot faster.
