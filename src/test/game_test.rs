use crate::osero::game;
use crate::osero::util;

const INITIAL_WHITE: u64 = 1 << 27 | 1 << 36;
const INITIAL_BLACK: u64 = 1 << 28 | 1 << 35;

#[test]
pub fn test_available_places() {
    let pair = (INITIAL_WHITE, INITIAL_BLACK);
    assert_eq!(
        game::available_places(&pair, &1),
        1 << 19 | 1 << 26 | 1 << 37 | 1 << 44
    );
    assert_eq!(
        game::available_places(&pair, &0),
        1 << 20 | 1 << 29 | 1 << 34 | 1 << 43
    );
}

#[test]
pub fn test_reverse_stone() {
    let pair = (INITIAL_WHITE, INITIAL_BLACK);
    let res = game::reverse_stone_new(&pair, &1, &(1 << 19));
    assert_eq!(res.0, 1 << 36);
    assert_eq!(res.1, 1 << 19 | 1 << 27 | 1 << 35 | 1 << 28);
}

#[test]
pub fn test_reverse_stone_killer() {
    let pair = (1 << 18 | 1 << 27 | 1 << 36, 1 << 19 | 1 << 28 | 1 << 35);
    let res = game::reverse_stone_new(&pair, &1, &(1 << 44));
    assert_eq!(res.0, 1 << 18 | 1 << 27);
    assert_eq!(res.1, 1 << 19 | 1 << 28 | 1 << 35 | 1 << 36 | 1 << 44);
}

#[test]
pub fn test_reverse_stone_killer2() {
    let pair = (
        0b_11111111_11100011_11110111_11001111_00011011_00111111_01111111_11111111,
        0b_00000000_00011100_00001000_00110000_11100100_01000000_00000000_00000000,
    );
    let res = game::reverse_stone_new(&pair, &0, &(1 << 15));
    assert_eq!(
        res.0,
        0b_11111111_11100111_11111111_11011111_00111011_01111111_11111111_11111111,
    );
    assert_eq!(
        res.1,
        0b_00000000_00011000_00000000_00100000_11000100_00000000_00000000_00000000,
    );
}

#[test]
pub fn test_available_places_killer() {
    let pair = (
        1 << 27 | 1 << 36 | 1 << 45,
        1 << 28 | 1 << 35 | 1 << 44 | 1 << 46 | 1 << 54 | 1 << 62,
    );
    let av = game::available_places(&pair, &0);
    println!(
        "{:?}",
        util::split_bit(&av)
            .iter()
            .map(|item| game::map_bit_index(&item))
            .collect::<Vec<u64>>()
    );
    assert_eq!(
        game::available_places(&pair, &0),
        1 << 20 | 1 << 29 | 1 << 34 | 1 << 43 | 1 << 47 | 1 << 52 | 1 << 63
    );
    assert_eq!(
        game::available_places(&pair, &1),
        1 << 18 | 1 << 19 | 1 << 26 | 1 << 37
    );
}
