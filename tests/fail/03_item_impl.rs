use pseudonym::alias;

#[alias(S1, S2)]
struct S(i32);

#[alias(
    S1,
    deprecated(S2),
    deprecated(S3, since = "0.1.0"),
    deprecated(S4, note = "deprecation note"),
    deprecated(S5, since = "0.1.0", note = "deprecation note"),
)]
impl S {
    fn new() -> Self {
        Self(5)
    }
}

fn main() {
    assert_eq!(S::new().0, 5);
    assert_eq!(S1::new().0, 5);
    assert_eq!(S2::new().0, 5);
    assert_eq!(S3::new().0, 5);
    assert_eq!(S4::new().0, 5);
    assert_eq!(S5::new().0, 5);
}