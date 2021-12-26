use pseudonym::alias;

#[alias(deprecated(f1), f2)]
fn a() {}

fn main() {
    f1()
}
