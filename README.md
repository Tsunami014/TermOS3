# TermOS
A terminal operating system for Kudos.

It's the third because I've started making 2 other terminal operating systems in different languages but they're all very incomplete.

## I just want to see it
```bash
cargo run --features bootloader
```

In Qemu press ctrl+alt+G to toggle input grabbing to input special keys properly like super

## Installing
Run these commands with `rustup` installed:
```bash
rustup component add rust-src
rustup component add llvm-tools-preview
cargo install bootimage
```
Also ensure `qemu` is installed

## Doing stuff with the code
`--features bootloader` will make the executable use the bootloader init. Useful when running without the KudOS main kernel, but for using with the kernel do not use it.

### Building (for use in kudos)
```bash
cargo build --release
```

### Running by itself
```bash
cargo run --features bootloader
```

### Testing (also by itself)
```bash
cargo test --features bootloader
```

### Updating
If KudOS changes and you want to update your version you're using:
```bash
cargo update
```
