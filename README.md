# rust_nickname_generater

![Crates.io](https://img.shields.io/crates/l/rust_nickname_generater)
![Crates.io](https://img.shields.io/crates/v/rust_nickname_generater)
[![docs](https://img.shields.io/badge/docs-github-blue)](https://buymymojo.github.io/rust_nickname_generater/rust_nickname_generater/)
[![docs](https://img.shields.io/badge/docs-docs.rs-blue)](https://docs.rs/rust_nickname_generater/)

[!["ay that's where the 'stralian accent comes through"](https://raw.githubusercontent.com/BuyMyMojo/rust_nickname_generater/master/images/TheReasonTheNameIsSpeltLikeThatOhMyThisIsALongFileName.png)](https://s3.buymymojo.net/ShareX/2022/08/01/17/rust%20nickname%20genera.wav)

> Yes I am australian :)

This is a super simple lib I made for practice.

The usernames generated are based on the names we all have in the [Serenity/Poise discord](https://discord.gg/serenity-rs) and the [rust community discord](https://discord.gg/rust-lang-communit)

## Basic use:

```rust
use rust_nickname_generater::generate_random_name;

// Generate a name that will fit in Discord
println!("{}", generate_random_name("mojo".to_string(), 32).unwrap());
```
