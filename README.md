## Dev

```
cargo clippy --all-features --tests --examples -- -D clippy::all
cargo +nightly clippy --all-features --tests --examples -- -D clippy::all

cargo fmt -- --check

cargo test-all-features -- --nocapture
```
