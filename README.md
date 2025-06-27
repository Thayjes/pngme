# PNG Steganography Tool

A command-line tool for hiding secret messages in PNG images using Rust.

## Features
- Hide text messages in PNG files
- Extract hidden messages
- List all chunks in a PNG
- Remove specific chunks

## Usage

cargo run -- encode your_image.png ruSt "Secret message!"
cargo run -- decode your_image.png ruSt
cargo run --print your_image.png
cargo run --remove your_image.png sRGB
