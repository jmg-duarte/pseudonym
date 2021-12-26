use pseudonym::alias;

#[alias(b, c,)]
fn a() -> i32 {
    5
}

fn main() {
    assert_eq!(a(), 5);
    assert_eq!(b(), 5);
    assert_eq!(c(), 5);
}