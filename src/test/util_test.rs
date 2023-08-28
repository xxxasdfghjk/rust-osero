use crate::osero::util;
#[test]
pub fn test_split_bit() {
    assert_eq!(util::split_bit(&0b_1111), vec![0b1, 0b10, 0b100, 0b1000]);
    assert_eq!(
        util::split_bit(&0b_1111001),
        vec![0b1, 0b1000, 0b10000, 0b100000, 0b1000000]
    );
}

#[test]
pub fn test_pop_count() {
    assert_eq!(util::pop_count(&1), 1);
    assert_eq!(util::pop_count(&3), 2);
    assert_eq!(util::pop_count(&7), 3);
    assert_eq!(util::pop_count(&15), 4);
    assert_eq!(util::pop_count(&16), 1);
    assert_eq!(util::pop_count(&17), 2);
    assert_eq!(util::pop_count(&(1 << 19)), 1);
}
