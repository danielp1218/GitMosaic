# GitMosaic

A small, fast command-line tool (Rust) for turning images into GitHub contribution graph art.

## Features
- Converts input images into a grid suitable for representing as GitHub contribution activity.
- Automates creating a repository and pushing timestamped commits to render the image on the contributions graph.

## Build

Install Rust (if needed) then build the release binary:

```sh
cargo build --release
```

## Run

Run the built binary (example):

```sh
# run from project root
./target/release/gitmosaic
```

