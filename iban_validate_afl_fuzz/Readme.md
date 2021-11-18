# How to use

To use this crate for fuzzing, use the (rust fuzzing book](https://rust-fuzz.github.io/book/afl.html). Basically:

```
cargo afl build
cargo afl fuzz -i inputs/ -o out ../target/debug/iban_validate_afl_fuzz
```

# Attribution
The inputs in the file `wikipedia` come from the Wikipedia page on IBAN's and as such are available under the CC BY-SA license.
