# Rust rewrite

Needed an excuse to learn Rust, so I'm rewriting my [personal website](https://clarke.gg) (currently written with Node.js using Express and Pug) in Rust using Actix and Askama.

## Stack
- Rust
    - Actix
    - Askama
- Node
    - Parcel (for bundling)

## Goals
- Complete port of JS version to Rust (lose 0 functionality)
- Unit-tested
- Minimal unsafe code
- Learn Rust along the way

## Running
- `yarn install` to install dependencies
- And either:
    - `yarn start` to start the server (bundles and then runs `cargo run`)
    - `yarn build` to build the frontend bundles and then runs `cargo build --release`)
