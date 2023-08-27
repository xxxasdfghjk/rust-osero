pub mod mymodule {
    pub struct Node {
        pub last_hand: u64,
        pub color: i32,
        pub point: f64,
        pub node_try_count: f64,
        pub boards: (u64, u64),
        pub children: Vec<Node>,
    }

    impl Node {
        pub fn is_leaf(&mut self) -> bool {
            self.children.len() == 0
        }
    }

    pub struct Tree {
        pub(crate) total_try_count: f64,
    }
    use std::cmp::Ordering;

    use rand::seq::SliceRandom;
    fn try_out(node: &Node) -> i32 {
        let mut current_turn = node.color;
        let mut boards = node.boards.clone();

        loop {
            let res = is_end(&boards);
            if res != -1 {
                return res;
            }

            let available_place = split_bit(&available_places(&boards, &current_turn));
            if available_place.len() == 0 {
                current_turn = if current_turn == 0 { 1 } else { 0 };
                continue;
            }
            boards = reverse_stone_new(
                &boards,
                &current_turn,
                available_place.choose(&mut rand::thread_rng()).unwrap(),
            );
            current_turn = if current_turn == 0 { 1 } else { 0 };
        }
    }

    const POP_MASK_0: u64 =
        0b_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101;

    const POP_MASK_1: u64 =
        0b_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011;
    const POP_MASK_2: u64 =
        0b_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111;
    const POP_MASK_3: u64 =
        0b_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111;
    const POP_MASK_4: u64 =
        0b_00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111;
    const POP_MASK_5: u64 =
        0b_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111;

    pub fn pop_count(num: &u64) -> u64 {
        let mut st = (*num & POP_MASK_0) + ((*num & (POP_MASK_0 << 1)) >> 1);
        st = (st & POP_MASK_1) + ((st & (POP_MASK_1 << 2)) >> 2);
        st = (st & POP_MASK_2) + ((st & (POP_MASK_2 << 4)) >> 4);
        st = (st & POP_MASK_3) + ((st & (POP_MASK_3 << 8)) >> 8);
        st = (st & POP_MASK_4) + ((st & (POP_MASK_4 << 16)) >> 16);
        st = (st & POP_MASK_5) + ((st & (POP_MASK_5 << 32)) >> 32);
        st
    }

    pub fn is_end(boards: &(u64, u64)) -> i32 {
        if pop_count(&available_places(boards, &0)) + pop_count(&available_places(boards, &1)) == 0
        {
            let wcnt = pop_count(&boards.0);
            let bcnt: u64 = pop_count(&boards.1);
            if wcnt > bcnt {
                0
            } else {
                1
            }
        } else {
            -1
        }
    }

    pub fn split_bit(bits: &u64) -> Vec<u64> {
        let mut cur = *bits;
        let mut res: Vec<u64> = vec![];
        loop {
            if cur == 0 {
                return res;
            } else {
                let right = cur & (!cur + 1);
                res.push(right);
                cur ^= right;
            }
        }
    }
    const MAX_NODE_TRY_COUNT: f64 = 500.0;
    const MAX_TRY_COUNT: i32 = 50000;

    pub fn tree_size(node: &Node) -> i32 {
        let mut res = 0;
        if node.children.len() == 0 {
            return 1;
        } else {
            for i in &node.children {
                res += tree_size(i);
            }
        }
        return res;
    }

    impl Tree {
        pub fn calc_next(&mut self, node: &mut Node) -> u64 {
            self.total_try_count = 0.0;
            let available = split_bit(&available_places(&node.boards, &node.color));
            node.children = available
                .iter()
                .map(|&pos| Node {
                    color: if node.color == 0 { 1 } else { 0 },
                    point: 0.0,
                    node_try_count: 0.0,
                    boards: reverse_stone_new(&node.boards, &node.color, &pos),
                    children: vec![],
                    last_hand: pos,
                })
                .collect::<Vec<Node>>();
            for _i in 1..MAX_TRY_COUNT {
                self.total_try_count += 1.0;
                self.traverse_node(node);
            }
            node.children.sort_by(|n1, n2| {
                if n2.point - n1.point > 0.0 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });

            for child in &node.children {
                println!("{} {}", child.point, map_bit_index(&child.last_hand));
            }
            println!("tree size :{}", tree_size(&node));
            node.children[0].last_hand
        }

        fn traverse_node(&mut self, node: &mut Node) -> i32 {
            if node.is_leaf() {
                let res = try_out(&node);
                node.node_try_count = node.node_try_count + 1.0;
                if res != node.color {
                    node.point = node.point + 1.0;
                }
                if node.node_try_count > MAX_NODE_TRY_COUNT {
                    let available = available_places(&node.boards, &node.color);
                    let sp = split_bit(&available);
                    node.children = sp
                        .iter()
                        .map(|&pos| Node {
                            color: if node.color == 0 { 1 } else { 0 },
                            point: 0.0,
                            node_try_count: 0.0,
                            boards: reverse_stone_new(&node.boards, &node.color, &pos),
                            children: vec![],
                            last_hand: pos,
                        })
                        .collect()
                }
                return res;
            } else {
                node.children.sort_by(|node1, node2| {
                    if calc_uct(self.total_try_count, node2.point, node2.node_try_count)
                        - calc_uct(self.total_try_count, node1.point, node1.node_try_count)
                        >= 0.0
                    {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });
                let res = self.traverse_node(&mut node.children[0]);
                node.node_try_count = node.node_try_count + 1.0;
                if res != node.color {
                    node.point = node.point + 1.0;
                }
                return res;
            }
        }
    }

    pub fn calc_uct(total_try_count: f64, point: f64, node_try_count: f64) -> f64 {
        return point / node_try_count
            + f64::sqrt(2.0) * f64::sqrt(f64::ln(total_try_count) / node_try_count);
    }

    pub fn print_board(boards: &(u64, u64)) {
        let INDEX: Vec<&str> = vec!["０", "１", "２", "３", "４", "５", "６", "７", ""];
        println!("　０１２３４５６７");
        print!("０");
        for i in 0..64 {
            if boards.0 & 1 << i > 0 {
                print!("⚫︎")
            } else if boards.1 & 1 << i > 0 {
                print!("⚪︎")
            } else {
                print!("＿")
            }
            if i % 8 == 7 {
                println!("");
                print!("{}", INDEX[(i + 1) / 8])
            }
        }
    }

    const RIGHT_HORIZONTAL_MASK: u64 =
        0b_01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111;
    const LEFT_HORIZONTAL_MASK: u64 =
        0b_11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110;

    //white = 0 , black = 1
    pub fn available_places(board: &(u64, u64), turn_color: &i32) -> u64 {
        let boards = if *turn_color == 1 {
            (board.0, board.1)
        } else {
            (board.1, board.0)
        };
        let mut available: u64 = 0;
        let mut tmp: u64;
        let no_exist_area = !(boards.0 | boards.1);
        // left
        tmp = ((boards.1 >> 1) & boards.0) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        available |= tmp & no_exist_area;

        // right
        tmp = ((boards.1 << 1) & boards.0) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        available |= tmp & no_exist_area;

        // down
        tmp = (boards.1 >> 8) & boards.0;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        available |= tmp & no_exist_area;

        // up
        tmp = (boards.1 << 8) & boards.0;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        available |= tmp & no_exist_area;

        // left up
        tmp = (boards.1 << 9) & boards.0 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        available |= tmp & no_exist_area;

        // right down
        tmp = (boards.1 >> 9) & boards.0 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        available |= tmp & no_exist_area;

        // right up
        tmp = (boards.1 << 7) & boards.0 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        available |= tmp & no_exist_area;

        // left down
        tmp = (boards.1 >> 7) & boards.0 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        available |= tmp & no_exist_area;
        available
    }

    pub fn reverse_stone_new(boards: &(u64, u64), turn_color: &i32, position: &u64) -> (u64, u64) {
        if (boards.0 | boards.1) & position == 1 {
            panic!("error!!")
        }
        if pop_count(position) != 1 {
            panic!("error!")
        }
        let mut boards = if *turn_color == 1 {
            (boards.0, boards.1)
        } else {
            (boards.1, boards.0)
        };

        let mut tmp: u64;
        // left
        tmp = ((boards.1 >> 1) & boards.0) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) >> 1) & RIGHT_HORIZONTAL_MASK;
        if tmp & position > 0 {
            let mut cur = position << 1;
            loop {
                if boards.1 & cur > 0 {
                    break;
                }
                boards.1 |= cur;
                boards.0 &= !cur;
                cur = cur << 1;
            }
        }

        // right
        tmp = ((boards.1 << 1) & boards.0) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        tmp |= ((tmp & boards.0) << 1) & LEFT_HORIZONTAL_MASK;
        if tmp & position > 0 {
            let mut cur = position >> 1;
            loop {
                if boards.1 & cur > 0 {
                    break;
                }
                boards.1 |= cur;
                boards.0 &= !cur;
                cur = cur >> 1;
            }
        }

        // down
        tmp = (boards.1 >> 8) & boards.0;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        tmp |= (tmp & boards.0) >> 8;
        if tmp & position > 0 {
            let mut cur = position << 8;
            loop {
                if boards.1 & cur > 0 {
                    break;
                }
                boards.1 |= cur;
                boards.0 &= !cur;
                cur = cur << 8;
            }
        }

        // up
        tmp = (boards.1 << 8) & boards.0;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        tmp |= (tmp & boards.0) << 8;
        if tmp & position > 0 {
            let mut cur = position >> 8;
            loop {
                if boards.1 & cur > 0 {
                    break;
                }
                boards.1 |= cur;
                boards.0 &= !cur;
                cur = cur >> 8;
            }
        }

        // left up
        tmp = (boards.1 << 9) & boards.0 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 9 & LEFT_HORIZONTAL_MASK;
        if tmp & position > 0 {
            let mut cur = position >> 9;
            loop {
                if boards.1 & cur > 0 {
                    break;
                }
                boards.1 |= cur;
                boards.0 &= !cur;
                cur = cur >> 9;
            }
        }

        // right down
        tmp = (boards.1 >> 9) & boards.0 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 9 & RIGHT_HORIZONTAL_MASK;
        if tmp & position > 0 {
            let mut cur = position << 9;
            loop {
                if boards.1 & cur > 0 {
                    break;
                }
                boards.1 |= cur;
                boards.0 &= !cur;
                cur = cur << 9;
            }
        }

        // right up
        tmp = (boards.1 << 7) & boards.0 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) << 7 & RIGHT_HORIZONTAL_MASK;
        if tmp & position > 0 {
            let mut cur = position >> 7;
            loop {
                if boards.1 & cur > 0 {
                    break;
                }
                boards.1 |= cur;
                boards.0 &= !cur;
                cur = cur >> 7;
            }
        }

        // left down
        tmp = (boards.1 >> 7) & boards.0 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        tmp |= (tmp & boards.0) >> 7 & LEFT_HORIZONTAL_MASK;
        if tmp & position > 0 {
            let mut cur = position << 7;
            loop {
                if boards.1 & cur > 0 {
                    break;
                }
                boards.1 |= cur;
                boards.0 &= !cur;
                cur = cur << 7;
            }
        }
        boards.1 |= position;
        return if *turn_color == 1 {
            (boards.0, boards.1)
        } else {
            (boards.1, boards.0)
        };
    }

    pub fn map_bit_index(bits: &u64) -> u64 {
        for i in 0..64 {
            if (1 << i) & bits > 0 {
                return i;
            }
        }
        64
    }
}

const INITIAL_WHITE: u64 = 1 << 27 | 1 << 36;
const INITIAL_BLACK: u64 = 1 << 28 | 1 << 35;
use std::io;

use mymodule::available_places;

use rand::seq::SliceRandom;

use mymodule::{Node, Tree};

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
    let mut board = (INITIAL_WHITE, INITIAL_BLACK);
    let mut current_color = 1;
    loop {
        let is_end = mymodule::is_end(&board);
        if is_end != -1 {
            mymodule::print_board(&board);
            println!("{} win !", if is_end == 0 { "white" } else { "black" });

            break;
        }
        let available = available_places(&board, &current_color);
        println!(
            "{} turn",
            if current_color == 0 { "white" } else { "black" }
        );
        if mymodule::pop_count(&available) == 0 {
            current_color = if current_color == 0 { 1 } else { 0 };
            continue;
        }
        mymodule::print_board(&board);
        loop {
            println!("input place [0..64):");
            println!(
                "{:?}",
                mymodule::split_bit(&available)
                    .iter()
                    .map(|item| mymodule::map_bit_index(item))
                    .collect::<Vec<u64>>()
            );

            let mut tree = Tree {
                total_try_count: 0.0,
            };
            let suggest = tree.calc_next(
                &mut (Node {
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
            board = mymodule::reverse_stone_new(&board, &current_color, &(input));
            current_color = if current_color == 0 { 1 } else { 0 };
            break;
        }
    }
}

fn main() {
    // let p = available_place(&pair, &1);
    // println!("{:?}", p);
    // let new_pair = reverse_stone_new(&pair, &1, &26);
    play();
    // mymodule::available_places(&pair, &0);
    return ();

    // let mut tree = Tree {
    //     total_try_count: 0.0,
    // };
    // let mut node = &mut Node {
    //     last_hand: (-1),
    //     color: (1),
    //     point: (0.0),
    //     node_try_count: (0.0),
    //     boards: (pair),
    //     children: (vec![]),
    // };
    // tree.calc_next(&mut node);
    // print_board(&pair);

    // print_board(&new_pair)
}
