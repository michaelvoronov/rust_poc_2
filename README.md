## rust_poc_2

```bash
cargo +nightly  build --target wasm32-wasi --release
```

produce this error message:
```bash
error[E0277]: the trait bound `proc_macro2::TokenStream: std::convert::From<proc_macro::TokenStream>` is not satisfied
   --> /Users/trofim/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-0.15.43/src/buffer.rs:109:27
    |
109 |         Self::new2(stream.into())
    |                           ^^^^ the trait `std::convert::From<proc_macro::TokenStream>` is not implemented for `proc_macro2::TokenStream`
    |
    = note: required because of the requirements on the impl of `std::convert::Into<proc_macro2::TokenStream>` for `proc_macro::TokenStream`

error[E0308]: mismatched types
    --> /Users/trofim/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-0.15.43/src/parse.rs:1078:52
     |
1078 |         self.parse2(proc_macro2::TokenStream::from(tokens))
     |                                                    ^^^^^^ expected struct `proc_macro2::TokenStream`, found struct `proc_macro::TokenStream`
     |
     = note: expected type `proc_macro2::TokenStream`
                found type `proc_macro::TokenStream`
```

But `cargo +nightly  build --target wasm32-unknown-unknown --release` succeed.
