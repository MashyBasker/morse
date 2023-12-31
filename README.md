# Morse

CLI utility to convert ASCII text to Morse code, made for educational and recreational purposes.

## How to use

- Install [Rust](rust-lang.org/)
- Clone the repo and `cd` into it

**For text mode**
```console
$ cargo run -- --mode text --target <text_you_want_to_convert>
```
**For file mode**
```console
$ cargo run -- --mode file --target <file_path>
```

## Plans for the future

- [ ] Make code [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself) by using a [PHF](https://docs.rs/phf/latest/phf/) as lookup table
- [ ] Add decryption utility
- [ ] Publish to `crates.io` as an application