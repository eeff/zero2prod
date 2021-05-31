# Zero To Production In Rust

This is my follow on the *Zero To Production [blog]* by [Luca Palmieri].
*Zero To Production* is an introduction to backend development using Rust,
with a focus on the challenges of writing Cloud-native applications.
It is a very good piece of writing which I learn a lot from.
The author also publishes it as a [book] which you can get early access.
The author also provides the [code] as supplementary material.

## Tool chains
* Rust
  * actix-web
  * reqwest
  * serde
  * sqlx
  * tracing
* postgres
* docker

## How to build

``` shell
cargo build
```

## How to test

``` shell
cargo test
```

[blog]: https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/
[book]: https://zero2prod.com/
[code]: https://github.com/LukeMathWalker/zero-to-production
[Luca Palmieri]: https://www.lpalmieri.com/about
