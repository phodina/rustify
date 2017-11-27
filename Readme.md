| Linux & Mac | [![Build status](https://travis-ci.org/phodina/rustify.svg?branch=master)](https://travis-ci.org/phodina/rustify)                                                     |
|-------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Windows     | [![Build status](https://ci.appveyor.com/api/projects/status/e9rkj7yeh04c6i2v/branch/master?svg=true)](https://ci.appveyor.com/project/phodina/rustify/branch/master) |

# Call C code in Rust Project

Template for porting/calling C code in Rust Project.

## Overview

There are two directories with the source codes.

* legacy - contains C code which is compiled into lib
* src - contains Rust code which is compiled as application

Generation of the C bindings for Rust is done on-the-fly by [Bindgen](https://crates.io/crates/bindgen) crate. The modules which should be compiled are speci

## Usage

To build the project simply run:

    cargo build

Here is example of several args you can pass:

    cargo run -- -h
    
    Say Hi
    Hello from C!

    cargo run -- -p

    Missing name of the place. Falling to default.
    We are at Prague!

    cargo run -- -p London

    We are at London!
    
    cargo run -- -m

    Multiply value: 6
    
## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

* **Petr Hodina** 

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
