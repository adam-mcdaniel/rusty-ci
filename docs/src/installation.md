# Installation

### Install Rust

To install Rusty-CI, you must install Rust.
You if you're on a Unix like platform, can do so by running this shell command.

```bash
curl https://sh.rustup.rs -sSf | sh
```

If you're on Windows, go to the [rust website](https://rust-lang.org). You'll need to download `rust-init.exe` and follow its instructions.

### Install Rusty-CI with Cargo (recommended)

Now that you have Rust, you should be able to install Rusty-CI by running the following command.

```bash
cargo install rusty-ci -f
```

This will automatically add Rusty-CI to your path, so we should be all done!

### Build from source (more tedious)

If you don't want to install with `cargo`, you can always build Rusty-CI from source. Here's how you would do so.

```bash
git clone https://github.com/adam-mcdaniel/rusty-ci

cd rusty-ci
# Cargo will output the executable to ./target/release/rusty-ci
cargo build --release

mv ./target/release/rusty-ci .
```

After running these commands, `rusty-ci` should be in your current working directory. To finish the installation, move the binary to any folder you'd like, and add that folder to your path.

### Problems Installing?

If you run into some problems installing Rusty-CI, or if Rust can't find `cc`, you need to upgrade some of the packages on your system.

Here's the commands I run to solve these errors.

```bash
sudo apt update
sudo apt upgrade
sudo apt install build-essential
```

After running these commands, try running `cargo install rusty-ci` again.

### Still having problems?

If you're still having some serious problems, [post an issue](https://github.com/adam-mcdaniel/rusty-ci/issues) on the repository.