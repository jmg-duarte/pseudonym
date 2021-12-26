# pseudonym 🕵️

Declare aliases for Rust constructs with ease!

```toml
[dependencies]
pseudonym = "0.2.0"
```

## Usage

The following example will generate a function
named `short_name` equal to `very_long_function_name`.

```rust
use pseudonym::alias;
#[alias(short_name)]
fn very_long_function_name() {
    // ...
}

fn main () {
    short_name(); // use the alias!
}
```

### Deprecating Aliases

Sometimes, you'll need to create aliases to old functions which are deprecated.
`pseudonym` allows you to deprecate aliases using the same syntax as `deprecated`.

```rust
use pseudonym::alias;
#[alias(
    deprecated(
        old_api_function,
        since = "0.1.0",
        note = "This function has been deprecated in favor of `new_api_function`"
    )
))]
fn new_api_function() {
    // ...
}

fn main () {
    short_name(); // use the alias!
}
```


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>