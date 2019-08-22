# Installation

### Install Python3

You're gonna need [python3](https://www.python.org/) to use Rusty-CI.

To install on a \*nix system, run the following commands.

```bash
apt update -y && apt upgrade -y
apt install -y build-essential python3-dev python3-pip python3-venv
```

You will need to be able to use `python3 -m pip` and `python3 -m venv`.

### Install Rust

To install Rusty-CI, you must install Rust.
You if you're on a Unix like platform, can do so by running this shell command.

```bash
curl https://sh.rustup.rs -sSf | sh
```

If you're on Windows, go to the [rust website](https://rust-lang.org). You'll need to download `rust-init.exe` and follow its instructions.

### Install Rusty-CI from Crates.io Package Registry (recommended)

Now that you have Rust, you should be able to install Rusty-CI by running the following command.

```bash
cargo install -f rusty-ci
```

This will automatically add Rusty-CI to your path, so we should be all done!

### Build from source (not recommended)

If you don't want to install Rusty-CI from the package registry, you can always build Rusty-CI from source. Here's how you would do so.

```bash
git clone https://github.com/adam-mcdaniel/rusty-ci

cd rusty-ci
# Cargo will output the executable to ./target/release/rusty-ci
cargo install -f --path .
```

This will automatically add Rusty-CI to your path, so we should be all done!

### Problems Installing?

If you run into some problems installing Rusty-CI, or if Rust can't find `cc`, you need to upgrade some of the packages on your system.

Here's the commands I run to solve these errors.

```bash
apt update -y && apt upgrade -y
sudo apt install build-essential
```

After running these commands, try running `cargo install rusty-ci` again.

### Can't find Cargo or Rusty-CI?

Try running the following to add Cargo and your installed crates to your environment's path.

```bash
source $HOME/.cargo/env # Add `cargo` to your path
```

### Still having problems?

If you're still having some serious problems, [post an issue](https://github.com/adam-mcdaniel/rusty-ci/issues) on the repository.
