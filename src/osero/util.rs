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

const POP_MASK_0: u64 = 0b_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101;
const POP_MASK_1: u64 = 0b_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011;
const POP_MASK_2: u64 = 0b_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111;
const POP_MASK_3: u64 = 0b_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111;
const POP_MASK_4: u64 = 0b_00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111;
const POP_MASK_5: u64 = 0b_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111;

pub fn pop_count(num: &u64) -> u64 {
    let mut st = (*num & POP_MASK_0) + ((*num & (POP_MASK_0 << 1)) >> 1);
    st = (st & POP_MASK_1) + ((st & (POP_MASK_1 << 2)) >> 2);
    st = (st & POP_MASK_2) + ((st & (POP_MASK_2 << 4)) >> 4);
    st = (st & POP_MASK_3) + ((st & (POP_MASK_3 << 8)) >> 8);
    st = (st & POP_MASK_4) + ((st & (POP_MASK_4 << 16)) >> 16);
    st = (st & POP_MASK_5) + ((st & (POP_MASK_5 << 32)) >> 32);
    st
}
