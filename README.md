## LLVM Examples

This repository contains _Rust_ examples using the great [inkwell](https://thedan64.github.io/inkwell/inkwell/) crate


### Tests:
For some reason, the tests pass **only** when running in a serial order, so make sure to use this command:
```zsh
cargo test -- --test-threads=1
```
