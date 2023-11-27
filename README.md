# SMMDB Client

![Continuous integration](https://github.com/Tarnadas/ninres-rs/workflows/Continuous%20integration/badge.svg)
![GitHub All Releases](https://img.shields.io/github/downloads/Tarnadas/smmdb-client/total)
![GitHub Releases](https://img.shields.io/github/downloads/Tarnadas/smmdb-client/latest/total)
[![Discord](https://img.shields.io/discord/168893527357521920?label=Discord&logo=discord&color=7289da)](https://discord.gg/SPZsgSe)
[![Twitter](https://img.shields.io/twitter/follow/marior_dev?style=flat&logo=twitter&label=follow&color=00acee)](https://twitter.com/marior_dev)

Save file editor for Super Mario Maker 2.

It will automatically detect your Yuzu and Ryujinx save folder, but you can also manually select any SMM2 save file on your system.

This software lets you download courses from [SMMDB](https://smmdb.net).
For planned features, please visit the [Github issue page](https://github.com/Tarnadas/smmdb-client/issues)

![](./assets/screenshot.png)

## Install

You can download Windows, Linux and MacOS binaries in the [Github release section](https://github.com/Tarnadas/smmdb-client/releases)

### via Cargo

You can install SMMDB Client via Cargo:

It is recommended to install Cargo via [Rustup](https://rustup.rs/)

#### Prerequisites (debian/ubuntu)

Before installing the client, run the following commands:

`sudo apt-get install cmake libfreetype6-dev libfontconfig1-dev xclip sudo libgtk-3-dev`

#### nightly install (all OSs)

After that, run these commands to fix rustup with nightly:

`rustup install nightly`

Set nightly as your default for now:

`rustup default nightly`

Now you can install the smmdb client:

`cargo install --git https://github.com/Tarnadas/smmdb-client.git`

Once you have installed smmdb, you can switch back to stable Rust:

`rustup default stable`

To open the smmdb client type `smmbd` in your terminal

### via Chocolatey (Windows Only)

`choco install smmdb-client`

Chocolatey install instructions/docs [Chocolatey.org](https://chocolatey.org/install)
