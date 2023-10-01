use crate::{SevendaysDatetimes, Calc};

#[test]
fn test_settime() {
    let sdtt = SevendaysDatetimes::new(7, 0, 0);
    assert_eq!(sdtt.to_settime(), Some(144000));
}