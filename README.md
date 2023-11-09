# zk_whitelist: A Zero Knowledge Whitelist Tool
![Test](https://github.com/SpiralOutDotEu/zk_whitelist/actions/workflows/rust_test.yml/badge.svg)
![GitHub Latest Release)](https://img.shields.io/github/v/release/SpiralOutDotEu/zk_whitelist?logo=github)


The Zero Knowledge Whitelist Tool is a powerful utility for managing an address whitelist using Zero-Knowledge (ZK) proofs. With this tool, you can effortlessly generate the necessary Solidity code for maintaining a secure and effective whitelist for your decentralized application.

> **DISCLAIMER**: This code and presentation is preliminary, **unaudited** and subject to revision. THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND.

## Benefits

* **No On-chain Updates Required**: Whitelisting new addresses does not require any on-chain updates. Just run the proofs command with the new addresses file.
* **Constant Proof Size**: The proof size remains constant regardless of the number of addresses, ensuring efficiency and scalability.
  
## Requirements

To utilize this tool, ensure that the following are installed on your system:

* [Node.js](https://nodejs.org/en)
* [Rust](https://www.rust-lang.org/)
* [circom](https://github.com/iden3/circom)
* [snarkjs](https://github.com/iden3/snarkjs)

## Recommended Usage

1) Download, extract and set as executable the binary for your OS from release page: 
   
   * https://github.com/SpiralOutDotEu/zk_whitelist/releases
  
2) Create a text file named `addresses.txt` and list the addresses you want to whitelist, one address per line.
```
0xdeadbeef...
0xbadcoffee...
```
3) Run the command `zk_whitelist all`. This will generate two Solidity files` verifier.sol` and `zkToken.sol`, alongside a `addresses.txt.proof.json` file.
```sh
./zk_whitelist all
```
4) Deploy your contracts and use the proofs file from your frontend to call the contract.
5) Whenever you need to whitelist additional addresses in the future, simply add them to a new file (e.g., `new_addresses.txt`), and run the command `zk_whitelist proofs --input-file "your_new_addresses_file"`. This will generate the necessary proofs for the new addresses and you can use them without any on-chain changes
```sh
./zk_whitelist proofs --input-file "your_new_addresses_file"
```

> Note: Besides `verifier.sol`, `zkToken.sol`, and `..proof.json`, all other generated files should be kept private and secure. Failing to secure these files could allow others to generate proofs on their own.

### Demo
[![Watch the video](https://img.youtube.com/vi/Sje7RINQMxU/hqdefault.jpg)](https://www.youtube.com/embed/Sje7RINQMxU)

## Alternative: Build from source

1) clone repo
```sh
git clone https://github.com/SpiralOutDotEu/zk_whitelist
```
2) Create a text file named `addresses.txt` and list the addresses you want to whitelist, one address per line.
```
0xdeadbeef...
0xbadcoffee...
```
3) Run the command `cargo run all`. This will generate two Solidity files` verifier.sol` and `zkToken.sol`, alongside a `..proof.json` file.
```sh
cargo run all
```
4) Deploy your contracts and use the proofs file from your frontend to call the contract.
* If you need to whitelist additional addresses in the future, simply add them to a new file (e.g., `new_addresses.txt`), and run the command `cargo run proofs --input-file "your_new_addresses_file"`. This will generate the necessary proofs for the new addresses and you can use them without any on-chain changes
```sh
cargo run proofs --input-file "your_new_addresses_file"
```

> Note: Besides `verifier.sol`, `zkToken.sol`, and `..proof.json`, all other generated files should be kept private and secure. Failing to secure these files could allow others to generate proofs on their own.

## Commands
Here are the available commands provided by this tool::

* `zk_whitelist --help`: show help docs
* `zk_whitelist circuit`: Creates a circuit file that controls the whitelisting to the current directory.
* `zk_whitelist compile`: Compiles the circuit file.
* `zk_whitelist setup`: Executes the setup ceremony to generate a secure setup.
* `zk_whitelist verifier`: Exports a Solidity verifier.
* `zk_whitelist movejs`: Moves the contents of circuit_js to the parent directory for convenience reasons.
* `zk_whitelist token`: Generates a sample token Solidity contract to be used together with the verifier.
* `zk_whitelist proofs --input-file "<input_file>"`: Generates proofs for a new set of addresses specified in <input_file>. Defaults to `addresses.txt` if no file is specified.
* `zk_whitelist all --input-file "<input_file>"`: Runs all the commands `(circuit, compile, setup, verifier, movejs, token, proofs)` one after the other. Defaults to `addresses.txt` if no file is specified.

## Contributing
Contributions are welcome! Feel free to submit a Pull Request or open an Issue for any bugs, enhancements, or new features.


---
*Leverage the power of zero-knowledge proofs to efficiently manage and update your address whitelist while ensuring the scalability and security of your decentralized application.*
