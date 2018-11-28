<img src="docs/logo.svg" width="200px" />

## NEAR Protocol - scalable and usable blockchain

![Build status](https://img.shields.io/gitlab/pipeline/nearprotocol/nearcore.svg)
![Discord](https://img.shields.io/discord/490367152054992913.svg)

NEAR Protocol is a new smart-contract platform that delivers scalability and usability.

Through sharding it aims to linearly scale with number of validation nodes on the network.

Leveraging WebAssembly, TypeScript, more sane contract management, ephemeral accounts and many other advancement, NEAR
makes using blockchain protocol for developers and consumers way easier compared to competitors.

## Building

### Setup rust

```bash
$ curl https://sh.rustup.rs -sSf | sh
$ rustup component add clippy-preview
```

You may need to activate the environment via `. ~/.cargo/env` to use `cargo`.

### Build from source code

```bash
# Download NEAR Core code.
git clone https://github.com/nearprotocol/nearcore
cd nearcore

cargo build --release
```

This produces an executable in the `./target/release` subdirectory.

### Simple Install for Mac and Linux

Coming soon.

### Start Node client

To start Near client manually after building, just run 

```bash
./target/release/nearcore
```

## DevNet

DevNet is a development tool that runs WebAssembly and State transition without running blockchain/consensus functionality.

To build and run it:

```bash
cd test-utils/devnet
cargo run
```

Try submitting transactions or views via JSON RPC:

```bash
# Send money:
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"receive_transaction","params":[{"nonce":1,"sender":1,"receiver":2,"amount":10,"method_name":"","args":[]}]}' 127.0.0.1:3030

# Submit contract
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"receive_transaction","params":[{"nonce":2,"sender":1,"receiver":123,"amount":0,"method_name":"deploy","args":[[<insert comma separated bytes of wasm code here>]]}]}' 127.0.0.1:3030

# View state
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"view","params":[{"account":1}]}' 127.0.0.1:3030
```

## Development

If you are planning to contribute, there are few more things to setup

### Setup git hooks

```bash
./scripts/setup_hooks.sh
```

### Setup rustfmt for your editor (optional)
Installation instructions [here](https://github.com/rust-lang-nursery/rustfmt#running-rustfmt-from-your-editor)

### Lints
We currently use [clippy](https://github.com/rust-lang-nursery/rust-clippy) to enforce certain standards.
This check is run automatically during CI builds, and in a `pre-commit`
hook. You can run do a clippy check with `./scripts/run_clippy.sh`.

## Documentation

Check out documentation at [http://docs.rs/nearcore](http://docs.rs/nearcore)
