use pseudonym::alias;

#[alias(
    T1,
    deprecated(T2),
    deprecated(T3, since = "0.1.0"),
    deprecated(T4, note = "deprecation note"),
    deprecated(T5, since = "0.1.0", note = "deprecation note")
)]
trait T {
    fn t(self);
}
fn main() {}