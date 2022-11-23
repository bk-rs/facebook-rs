## Dev

```
cargo clippy --all-features --tests -- -D clippy::all
cargo +nightly clippy --all-features --tests -- -D clippy::all

cargo fmt -- --check

cargo build-all-features
cargo test-all-features -- --nocapture
```

## Publish order

facebook-permission

facebook-signed-request

facebook-webhook facebook-webhook-warp

facebook-fb-login-deauth-callback facebook-fb-login-deauth-callback-warp
