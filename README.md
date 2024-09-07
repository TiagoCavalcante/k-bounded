# k-bounded functions count

This repository contains the Rust implementation for computing the number of k-bounded functions from [n] to [n]. The computation is based on matrix operations over big integers to handle the large numbers that occur in calculations for larger values of n and k.

## Description

The main logic of this program is to compute the number of k-bounded functions using a matrix representation of the problem. It constructs a matrix where each entry is determined by a sigma function which checks if the absolute difference between indices is within the k threshold. The program then raises this matrix to the power of n-1 and multiplies it by a vector of ones to get the final result.

## Requirements

To run this code, you will need Rust installed on your machine. You can download and install Rust from [the official Rust website](https://www.rust-lang.org/tools/install).

## Usage

1. Clone this repository:
```sh
git clone https://github.com/TiagoCavalcante/k-bounded
```
2. Navigate into the repository directory:
```sh
cd k-bounded
```
3. Build and run the program:
```sh
cargo build --release
```
