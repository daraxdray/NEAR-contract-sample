#!/bin/bash

# near delete daraxeclat.testnet daraxeclat.testnet
# near create-account --masterAccount daraxeclat.testnet
near deploy daraxeclat.testnet --wasmFile=./res/counter.wasm