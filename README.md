# rustchain
Basic command line wrapper written in Rust for the blockchain wallet api, requires UNIX/nightly compiler

Must have blockchain wallet serivice installed, see: https://github.com/blockchain/service-my-wallet-v3#installation

# Usage

Run the program and proceed through the steps, automatically manages api service

# Installation

Clone this repository, build with nightly compiler and run (--release preferred), untested on non-UNIX systems

# TODO
- refactor JSON decoding into method, sloppy handling currently
- command line args vs. user input
- handle multiple wallet files
- implement remaining api functionality
- some semblance of decent error checking 
- encrypt binary wallet data(?)
