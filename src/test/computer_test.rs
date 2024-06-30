use rand::seq::IteratorRandom;

use crate::{
    computer::{self, convert_num_to_cell},
    osero::{self, util::split_bit},
};
pub const INITIAL_WHITE: u64 = 1 << 27 | 1 << 36;
pub const INITIAL_BLACK: u64 = 1 << 28 | 1 << 35;
fn computer_vs_monkey() -> i32 {
    let mut board: (u64, u64) = (INITIAL_WHITE, INITIAL_BLACK);
    let mut current_color = 1;
    let mut history = "".to_string();
    let computer_color = 1;
    let monkey_color = 0;

    loop {
        let is_end = osero::game::is_end(&board);
        if is_end != -1 {
            osero::game::print_board(&board);
            println!("{} win !", if is_end == 0 { "white" } else { "black" });
            return is_end;
        }
        let available = osero::game::available_places(&board, &current_color);
        if osero::util::pop_count(&available) == 0 {
            current_color = if current_color == 0 { 1 } else { 0 };
            continue;
        }
        osero::game::print_board(&board);
        if current_color == monkey_color {
            let &next_hand = split_bit(&available)
                .iter()
                .choose(&mut rand::thread_rng())
                .unwrap();
            board = osero::game::reverse_stone_new(&board, &current_color, &next_hand);
            history.push_str(&convert_num_to_cell(next_hand));
            println!("monkey set")
        } else {
            let mut tree = computer::Tree {
                total_try_count: 0.0,
            };
            let suggest = tree.calc_next(
                &mut (computer::Node {
                    last_hand: 0,
                    color: current_color,
                    point: 0.0,
                    node_try_count: 0.0,
                    boards: board,
                    children: vec![],
                }),
                Some(500),
                &history,
            );
            board = osero::game::reverse_stone_new(&board, &current_color, &suggest);
            history.push_str(&convert_num_to_cell(suggest));
            println!("computer set")
        }
        current_color = if current_color == 0 { 1 } else { 0 };
    }
}
#[test]
fn computer_vs_monkey_multi() {
    let mut cnt = 0;
    let MAX = 10;
    for i in 0..MAX {
        cnt += if computer_vs_monkey() == 1 { 1 } else { 0 };
        println!("{} : {}", i, cnt);
    }
    println!("{}", cnt / MAX)
}
