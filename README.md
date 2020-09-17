# geforcedrvchk3

GeForceDrvChk is a small no-nonsense application for automatically checking Geforce driver updates under Windows.

## Introduction

This little piece of code checks the GeForce website for new driver versions. It only checks for the International GeForce GTX series driver for 64-bit Windows desktop (so, no laptop version of the driver).

The main point of the application is to prove myself that I'm able to implement everything required using only Rust. Of course, it also serves me as a replacement for GeForce Experience.

So far, the following goals have been implemented:

- calling a console application and catching the output
- using regex
- fetching a page from a WWW server over SSL
- fetching information from json data
- compiling single statically linked binary without any dependencies
- optional automatic downloading and installation
- unit tests

Possible future goals:

- using MessageBoxW via FFI and Win32 API

## Usage

### Compiling from source code

1. clone the project
1. install Rust: [instructions from the official site](https://www.rust-lang.org/learn/get-started)
1. go to the project folder and run: `cargo build --release`
1. fetch compiled binary from the target\release folder

Alternatively download the pre-built binary from here: [geforcedrvchk3.exe](https://github.com/mjkarki/geforcedrvchk3/releases/download/v0.3.2/geforcedrvchk3.exe) (at [github.com](https://github.com/mjkarki/geforcedrvchk3/releases))

### Installing

1. place the geforcedrvchk3.exe somehwere (e.g. `%UserProfile%`)
1. open the Windows file exporer and navigate to `shell:startup`
1. create shortcut to the geforcedrvchk3.exe

### Attention

Version 0.2 (and later versions) has a new experimental feature: automatic driver download and installation. By default geforcedrvchk3 still uses the web browser to download the driver (started by pressing ENTER or by selecting 'D' from the options). The new addition is the option 'A', which tries to download the driver file to the %TEMP% and execute it with options, which only performs the Display.Driver module installation and automatic restarting. This feature is somewhat undocumented and may result in unexpected behavior (and that's the reason for the forced reboot). The feature seems to work fine, if there already is an existing driver installation with working configuration at the system.

If you use the automatic installation option, please ensure that all other programs are closed before starting!

## License

geforcedrvchk3 is licensed under the BSD 3-Clause "New" or "Revised" License.
