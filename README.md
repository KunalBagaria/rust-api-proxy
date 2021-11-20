# Rust API Proxy
A simple proxy to save an API request in memory every 10 seconds and serve it again

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/new/template?template=https%3A%2F%2Fgithub.com%2Fkb24x7%2Frust-api-proxy&referralCode=7OsmmG)

Change the GET API URL you want to proxy in [main.rs](https://github.com/kb24x7/rust-api-proxy/blob/main/src/main.rs#L20)

```rust
let res = client.get("https://api.coingecko.com/api/v3/coins/solana").send().await?;
```


or the duration in [main.rs](https://github.com/kb24x7/rust-api-proxy/blob/main/src/main.rs#L34)

```rust
periodic::Every::new(Duration::from_secs(10))
```
