use pseudonym::alias;

#[alias(
    S1,
    deprecated(S2),
    deprecated(S3, since = "0.1.0"),
    deprecated(S4, note = "deprecation note"),
    deprecated(S5, since = "0.1.0", note = "deprecation note")
)]
struct S(i32);

fn main() {
    assert_eq!(S(5).0, 5);
    assert_eq!(S1(5).0, 5);
    assert_eq!(S2(5).0, 5);
    assert_eq!(S3(5).0, 5);
    assert_eq!(S4(5).0, 5);
    assert_eq!(S5(5).0, 5);
}
