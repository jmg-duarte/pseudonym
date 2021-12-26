use pseudonym::alias;

#[alias(S1, S2)]
struct S (i32);

fn main() {
    assert_eq!(S(5).0, 5);
    assert_eq!(S1(5).0, 5);
    assert_eq!(S2(5).0, 5);
}