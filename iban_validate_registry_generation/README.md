# iban_validate_registry_generation
This crate can generate the repetitive country-specific code from the IBAN registry.

The code tries to read the registry file from the local directory, and automatically generates the files `src/generated` for country specific code and `tests/registry_examples_generated.rs` for country specific tests. There are quite some errors and inconsistencies in the registry, so they cannot be used as tests directly. We try to fix them or mark them as unusable. See [`fix_inconsistencies`] for details.
