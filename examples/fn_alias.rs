use pseudonym::alias;

#[alias(S1, S2)]
struct S(i32);

#[alias(S1, S2)]
impl S {
    fn new() -> Self {
        Self(5)
    }
}

fn main() {
    assert_eq!(S::new().0, 5);
    assert_eq!(S1::new().0, 5);
    assert_eq!(S2::new().0, 5);
}