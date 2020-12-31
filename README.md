# Zeropool Substrate (WIP)

Zeropool Substrate contains the modules for Substrate (Polkadot) of Zeropool.


## Zeropool/Substrate Dev Node
zeropool-substrate-devnet folder contains a working single node development network with zeropool-substrate pallet.

This is proof of concept of verifier of zkSNARKs on PolkaDot blockchain. `groth16verify` is implemented and could be tested via `test_groth16verify` external pallet method.

Test vectors could be obtained from [fawkes-crypto](https://github.com/zeropoolnetwork/fawkes-crypto/blob/master/fawkes-crypto/src/backend/bellman_groth16/mod.rs#L204) or other zkSNARK frameworks.

### Requirements
Install Rust compiler: https://www.rust-lang.org
Install Make utility: https://www.gnu.org/software/make/
### How to Build and Run
To build and run the dev node, execute:
```bash
cd zeropool-substrate-devnet
make init
make run
```
### Pallet Unit Test
```bash
cd zeropool-substrate-devnet/pallets/zeropool-substrate
cargo +nightly-2020-10-05 test --release
```

## Zeropool/Substrate Nodejs Client
zeropool-substrate-nodejs-client contains a testing unit interacting with the Zeropool/Substrate node.
### Requirements
Install nodejs: https://nodejs.dev
Install yarn: https://yarnpkg.com
### How to Build and Run
To build and connect to the running dev node, execute:
```bash
cd zeropool-substrate-nodejs-client
yarn add @polkadot/api
node testapp.js
```

## Zeropool/Substrate Web Client
zeropool-substrate-client contains a basic web client to interact with the Zeropool/Substrate node.
### Requirements
Install yarn: https://yarnpkg.com
### How to Build and Run
To build and connect to the running dev node, execute:
```bash
cd zeropool-substrate-client
yarn install
yarn start
```
