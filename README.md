# yts-cli

A simple CLI to list movies from YTS (https://yts.mx) and generate magnet links.

how to install (it doesn't work because why would it want to?)
```bash
git clone https://github.com/Ka-derka/yts-cli.git
cd yts-cli
makepkg -si
```

or how to _acutally_ install it

```bash
git clone https://github.com/Ka-derka/yts-cli.git
cd yts-cli
cargo build --release
sudo install -Dm755 target/release/yts-cli /usr/local/bin/yts-cli
