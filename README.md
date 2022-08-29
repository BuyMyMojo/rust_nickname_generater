# rust_nickname_generater

This is a super simple lib I made for practice.

The usernames generated are based on the names we all have in the [Serenity/Poise discord](https://discord.gg/serenity-rs) and the [rust community discord](https://discord.gg/rust-lang-communit)

## Basic use:

```rust
use rust_nickname_generater::generate_random_name;

// Generate a name that will fit in Discord
println!("{}", generate_random_name("mojo".to_string(), 32).unwrap());
```

License: Apache-2.0