pub fn is_end(boards: &(u64, u64)) -> i32 {
    if super::util::pop_count(&available_places(boards, &0))
        + super::util::pop_count(&available_places(boards, &1))
        == 0
    {
        let wcnt = super::util::pop_count(&boards.0);
        let bcnt: u64 = super::util::pop_count(&boards.1);
        if wcnt > bcnt {
            0
        } else {
            1
        }
    } else {
        -1
    }
}

const MAX_NODE_TRY_COUNT: f64 = 500.0;
const MAX_TRY_COUNT: i32 = 50000;

pub fn print_board(boards: &(u64, u64)) {
    let index: Vec<&str> = vec!["０", "１", "２", "３", "４", "５", "６", "７", ""];
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
            print!("{}", index[(i + 1) / 8])
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
    if super::util::pop_count(position) != 1 {
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
