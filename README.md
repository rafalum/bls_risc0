# BLS Public Key Aggregation in RiscZero

This is a little toy example to prove that we know `n` BLS public keys that result in a specific aggregated key.

## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

To create a receipt (i.e. a proof of knowledge of `n` BLS public keys) run:

```bash
cargo run --bin host
```

To verify the receipt run:

```bash
cargo run --bin verify
```