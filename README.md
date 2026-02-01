# Template
A template for other apps to use KudOS

## Updating
If KudOS changes and you want to update your version you're using:
```bash
cargo update
```

## Doing stuff with the code
`--features bootloader` will make the executable use the bootloader init. Useful when running without the KudOS main kernel, but for using with the kernel do not use it.
### Building
```bash
cargo build --release
```
### Running
```bash
cargo run --features bootloader
```
### Testing
```bash
cargo test --features bootloader
```
