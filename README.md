# pseudonym 🕵️

Declare aliases for Rust constructs with ease!

```toml
[dependencies]
pseudonym = "0.2.1"
```

## Usage
In the following examples the items to which the macro is attached to
get generated as aliases using the passed identifier.

### Functions

```rust
# use pseudonym::alias;
#[alias(short_name)]
fn very_long_function_name() {
    // ...
}

fn main () {
    short_name(); // use the alias!
}
```

### Structures

```rust
# use pseudonym::alias;
#[alias(StructAlias)]
struct S (i32);
```

### Traits

```rust
# use pseudonym::alias;
#[alias(TraitAlias)]
trait T {}
```

### Implementations

```rust
# use pseudonym::alias;
#[alias(StructAlias)]
struct S;

#[alias(StructAlias)]
impl S {
    fn new() -> Self { S }
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