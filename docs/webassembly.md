## build rust

```sh
(
    cd rust
    wasm-pack build --release --target web
    cp -rf pkg ../frontend
)
```

## run

```sh
(
    cd frontend
    npm run dev
)
```