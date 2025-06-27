# PNG Steganography Tool

A command-line tool for hiding secret messages in PNG images using Rust.

## Features
- Hide text messages in PNG files
- Extract hidden messages
- List all chunks in a PNG
- Remove specific chunks

## Installation
1. Clone the repository: `git clone https://github.com/Thayjes/pngme.git`
2. Navigate to the directory: `cd pngme`
3. Build the project: `cargo build --release`

## Usage

- `cargo run -- encode your_image.png ruSt "Secret message!"`
- `cargo run -- decode your_image.png ruSt`
- `cargo run -- print your_image.png`
- `cargo run -- remove your_image.png sRGB`

## References
https://jrdngr.github.io/pngme_book/
