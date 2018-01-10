# Rust Replicate

[![Join the chat at https://gitter.im/rust-rep/Lobby](https://badges.gitter.im/rust-rep/Lobby.svg)](https://gitter.im/rust-rep/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Build Status](https://travis-ci.org/NathanFrasier/rust-rep.svg?branch=master)](https://travis-ci.org/NathanFrasier/rust-rep)

Rust Replicate is a toy implementation of a Peer-to-peer style file sharing system. It uses Rust to implement largely incorrect and inefficient LT codes to be used for file replication.

## Installing / Getting started

Installation requires building the source. Once built you can use

```shell
git clone https://github.com/NathanFrasier/rust_rep.git
cd rust-rep/
cargo build
./target/debug/rust-rep --version # prints the verison
```

## Features

`rust-rep` allows for peer to peer file replication using a fountain code implementation
* `rust-rep serve` can be used to serve a file for replication
* `rust-rep seed` can be used to replicate a file from another instance of `rust-rep` and then act as a peer download source.
* `rust-rep leech` can be used to simply copy a file from another `rust-rep` instance

## Configuration



#### -f \<filename>
Type: `String`

Specifies the file to allow replication of.


#### -c \<connectionInfo>
Type: `String`

Describes the connection info that can be used to reach this agent (usually ip:port)

#### -s \<serverConnectionInfo>
Type: `String`

Describes the connection info of another `rust-rep` instance (usually ip:port)

#### -o [outputFile]
Type: `String`  
Default: "out.txt"

What the file output should be named.

#### -h --help  

Display help dialog

#### -v --version  

Display version info

## Contributing

I don't really expect anyone to contribute here. This is really meant to be a pet project for me to learn Rust and some basics of fountain codes in practice. If you'd like to contribute, please fork the repository and use a feature branch. Pull requests are welcome.

## Licensing
The code in this project is licensed under MIT license.
