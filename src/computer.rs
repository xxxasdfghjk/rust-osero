use std::cmp::Ordering;
use std::path::Path;
use std::path::PathBuf;

use crate::book::Book;
use crate::osero::game::*;
use crate::osero::util::*;
use rand::prelude::SliceRandom;
use rand::seq::IteratorRandom;
use rusqlite::params;
use rusqlite::Connection;
const MAX_NODE_TRY_COUNT: f64 = 100.0;
const MAX_TRY_COUNT: i32 = 50000;
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
    pub total_try_count: f64,
}

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

fn get_relative_path(relative_path: &str) -> PathBuf {
    let current_file_path = Path::new(file!());
    let current_dir = current_file_path.parent().unwrap();
    current_dir.join(relative_path)
}

fn convert_cell_to_num(cell: &str) -> u64 {
    let cell = match cell.chars().nth(0) {
        Some('A') => 0,
        Some('B') => 1,
        Some('C') => 2,
        Some('D') => 3,
        Some('E') => 4,
        Some('F') => 5,
        Some('G') => 6,
        Some('H') => 7,
        _ => panic!(),
    } + match cell.chars().nth(1) {
        Some('1') => 0,
        Some('2') => 8,
        Some('3') => 16,
        Some('4') => 24,
        Some('5') => 32,
        Some('6') => 40,
        Some('7') => 48,
        Some('8') => 56,
        _ => panic!(),
    };
    return 1 << cell;
}

pub fn most_right_bit_index(bits: u64) -> i32 {
    for i in 0..64 {
        if bits & (1 << i) > 0 {
            return i;
        }
    }
    return -1;
}

pub fn convert_num_to_cell(num: u64) -> String {
    let index = most_right_bit_index(num);
    return format!(
        "{}{}",
        "ABCDEFGH"
            .chars()
            .nth(usize::try_from(index % 8).unwrap())
            .unwrap(),
        "12345678"
            .chars()
            .nth(usize::try_from(index / 8).unwrap())
            .unwrap()
    );
}

pub fn get_next_hand_from_book(history: &str) -> Option<u64> {
    let path = "./../assets/book";
    let db_path = get_relative_path(path);
    let con = Connection::open(&db_path).unwrap();
    let mut statement = con
        .prepare("select id,name,moves from book where moves like ?1")
        .unwrap();
    let res = statement
        .query_map(params![format!("{history}%")], |row| {
            Ok(Book {
                name: row.get(1).unwrap(),
                id: row.get(0).unwrap(),
                moves: row.get(2).unwrap(),
            })
        })
        .unwrap()
        .filter_map(Result::ok)
        .filter(|x| x.moves.len() > history.len())
        .collect::<Vec<Book>>();
    let mut rng = rand::thread_rng();
    let choice = res.choose(&mut rng);
    println!(
        "book founded : {} count , hisotry : {} ",
        res.len(),
        history
    );
    if let Some(book) = choice {
        println!("book choice : {}", book.name);
        return Some(convert_cell_to_num(&book.moves[history.len()..][0..2]));
    } else {
        return None;
    }
}

impl Tree {
    pub fn calc_next(&mut self, node: &mut Node, max_try_count: Option<i32>, history: &str) -> u64 {
        let book_hand = get_next_hand_from_book(history);
        if let Some(hand) = book_hand {
            return hand;
        }
        let max_try_count = match max_try_count {
            Some(val) => val,
            None => MAX_TRY_COUNT,
        };
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
        for _i in 1..max_try_count {
            self.total_try_count += 1.0;
            self.traverse_node(node);
        }
        node.children.sort_by(|n1, n2| {
            if n2.point / n2.node_try_count - n1.point / n2.node_try_count > 0.0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        for child in &node.children {
            println!(
                "{} {}",
                child.point / child.node_try_count,
                map_bit_index(&child.last_hand)
            );
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
