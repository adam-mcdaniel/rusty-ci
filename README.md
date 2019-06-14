# rusty-ci

A tool to generate buildbot projects from a YAML file

## Usage

Install rust.

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install `rusty-ci`

```bash
cargo install rusty-ci
```

Build a makefile from a yaml file and run it!

```bash
cat test.yaml | rusty-ci > Makefile
make -j
```