# bevyjam2
Bevy Jam 2

# Cross compilation from mac:

### Install mingw-w64 to support linker.

```shell
brew install mingw-w64
```

Note, you may need to run the following, depending on your brew setup:

```shell
arch -arm64 brew install mingw-w64
```

### Commands to cross-compile
```shell
cargo build --target=aarch64-apple-darwin --verbose --release && cargo build --target=x86_64-apple-darwin --verbose --release && cargo build --target=x86_64-pc-windows-gnu --verbose --release && echo "FINISHED BUILD"
```