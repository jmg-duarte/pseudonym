use pseudonym::alias;

#[alias(
    b,
    deprecated(c),
    deprecated(d, since = "0.1.0"),
    deprecated(e, note = "deprecation note"),
    deprecated(f, since = "0.1.0", note = "deprecation note")
)]
const a: usize = 5;

fn main() {
    assert_eq!(a, 5);
    assert_eq!(b, 5);
    assert_eq!(c, 5);
    assert_eq!(d, 5);
    assert_eq!(e, 5);
    assert_eq!(f, 5);
}
