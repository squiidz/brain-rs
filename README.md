# brain
brainfuck interpreter in rust

## Run Cli
- `cd brain`
- `cargo run brainfuck/hello_world.b`

## Run Webapp
- `cd brain/web_brain`
- `cargo run`

## Run Ruby
- `CONFIGURE_OPTS=--enable-shared rbenv install 2.3.x`
- `cd brain`
- `cargo build --release`
- `cd brain/ruby_brain/ruby`
- `ruby test_ruby_brain.rb`
