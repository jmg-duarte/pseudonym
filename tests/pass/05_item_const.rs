use pseudonym::alias;

#[alias(
    C1,
    deprecated(C2),
    deprecated(C3, since = "0.1.0"),
    deprecated(C4, note = "deprecation note"),
    deprecated(C5, since = "0.1.0", note = "deprecation note")
)]
const C: u8 = 1;
fn main() {}