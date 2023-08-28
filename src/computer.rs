use std::cmp::Ordering;

use crate::osero::game::*;
use crate::osero::util::*;
use rand::prelude::SliceRandom;
const MAX_TRY_COUNT: i32 = 50000;
const MAX_NODE_TRY_COUNT: f64 = 500.0;

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
