use testing::{sploosh, splish};

#[test]
fn ensure_composition() {
  assert_eq!(4, sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)))
}