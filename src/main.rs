const INITIAL_WHITE: u64 = 1 << 27 | 1 << 36;
const INITIAL_BLACK: u64 = 1 << 28 | 1 << 35;
use rust_osero::computer;
use rust_osero::osero;
use std::io;
fn read_buffer() -> u32 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().parse().expect("Failed to parse"),
        Err(e) => panic!("Failed to read line: {}", e),
    }
}

fn read_buffer_str() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().parse().expect("Failed to parse"),
        Err(e) => panic!("Failed to read line: {}", e),
    }
}

fn play() {
    let mut board: (u64, u64) = (INITIAL_WHITE, INITIAL_BLACK);
    let mut current_color = 1;
    loop {
        let is_end = osero::game::is_end(&board);
        if is_end != -1 {
            osero::game::print_board(&board);
            println!("{} win !", if is_end == 0 { "white" } else { "black" });

            break;
        }
        let available = osero::game::available_places(&board, &current_color);
        println!(
            "{} turn",
            if current_color == 0 { "white" } else { "black" }
        );
        if osero::util::pop_count(&available) == 0 {
            current_color = if current_color == 0 { 1 } else { 0 };
            continue;
        }
        osero::game::print_board(&board);
        loop {
            println!("input place [0..64):");
            println!(
                "{:?}",
                osero::util::split_bit(&available)
                    .iter()
                    .map(|item| osero::game::map_bit_index(item))
                    .collect::<Vec<u64>>()
            );

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
            );
            let input = if current_color == 1 {
                1 << read_buffer() as u64
            } else {
                suggest
            };
            board = osero::game::reverse_stone_new(&board, &current_color, &(input));
            current_color = if current_color == 0 { 1 } else { 0 };
            break;
        }
    }
}

fn main() {
    play();
    return ();
}
