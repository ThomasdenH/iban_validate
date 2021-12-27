# iban_validate_registry_generation
This crate can generate the repetitive country-specific code from the IBAN registry.

The code could be better, but honestly, it's not worth it. The txt format is
undocumented, ill-structured and contains quite a few errors.

The errors found so far:
- The bank identifier range for Albania is `1-3`, which does not match the length `8!n`. The example and national documentation suggests the length should be used.
- Also for Albania, the branch identifier range is `4-8`, but it should be `4-7`. It seems that the range should be exclusive, where all others are inclusive.
- The bank identifier range for SI is malformed. I believe the range was terminated by a newline, which a formatter then surrounded by `"_"` to escape.
- The bank identifier range for Jordan is just incorrect, judging from national documentation and the example in the registry.
- The example IBAN for Sao Tome and Principe is not a valid IBAN; its checksum is incorrect.
- For BI, LY, SV, VA, the pretty print IBAN is not formatted per the standard.

Aside from that, the examples can be used to generate tests in most cases. However, there are instances where this is not possible. Most of the time, all examples are part of the same IBAN. For some countries that isn't the case. Other cases:
- Brasil has an IBAN where it's BBAN part has one character different to the example IBAN:
    - `BR1800360305000010009795493C1`
    - `    00360305000010009795493P1`
- Israel has an extra 0 in the IBAN:
    - `IL620108000000099999999`
    - `    010800000099999999`
- For Costa Rica, there is just a leading 0 missing from the IBAN. This is not really a mistake but does make the comparison impractical.
