# Zeropool Substrate 

Zeropool Substrate contains the modules for Substrate (Polkadot) of Zeropool.

## Zeropool/Substrate Dev Node
zeropool-substrate-devnet folder contains a working single node development network with zeropool-substrate pallet.
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
