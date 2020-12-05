# Zeropool Substrate Node 

Zeropool substrate development network


### How to Build

1. `make init` - Run the [init script](scripts/init.sh) to configure the Rust toolchain for
   [WebAssembly compilation](https://substrate.dev/docs/en/knowledgebase/getting-started/#webassembly-compilation).
1. `make run` - Build and launch this project in development mode.

The init script and Makefile both specify the version of the
[Rust nightly compiler](https://substrate.dev/docs/en/knowledgebase/getting-started/#rust-nightly-toolchain)
that this project depends on.

### Build

The `make run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
make build
```

### Help

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/zeropool-substrate-devnet -h
```

## Run

The `make run` command will launch a temporary node and its state will be discarded after you
terminate the process. After the project has been built, there are other ways to launch the node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/zeropool-substrate-devnet --dev 
```

This command will start the single-node development chain with temporary state:
```bash
./target/release/zeropool-substrate-devnet --dev --tmp
```


Purge the development chain's state:

```bash
./target/release/zeropool-substrate-devnet purge-chain --dev 
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/zeropool-substrate-devnet -lruntime=debug --dev
```
### Unit Testing for zeropool-substrate pallet
Execute:
```bash
cd zeropool-substrate-devnet/pallets/zeropool-substrate
cargo +nightly-2020-10-05 test  --features=borsh_support
```
