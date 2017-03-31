# brain [![crates.io](https://img.shields.io/crates/v/brain-rs.svg)](https://crates.io/crates/brain-rs) [ ![Codeship Status for squiidz/brain-rs](https://app.codeship.com/projects/f73e4a40-ec7b-0134-81d5-1ac2cf405306/status?branch=master)](https://app.codeship.com/projects/208334)

brainfuck interpreter in rust

![alt tag](http://data.whicdn.com/images/57083921/large.jpg)

## Run Cli
- `cd brain`
- `cargo run -- brainfuck/hello_world.b`

## Run Webapp
- `cd brain/web_brain`
- `cargo run`

## Run Ruby
- `CONFIGURE_OPTS=--enable-shared rbenv install 2.3.x`
- `cd brain`
- `cargo build --release`
- `cd brain/ruby_brain/ruby`
- `ruby test_ruby_brain.rb`
