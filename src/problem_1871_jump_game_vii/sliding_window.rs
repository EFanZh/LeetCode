pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let mut s = s.into_bytes();
        let min_jump = min_jump as u32 as usize;
        let max_jump = max_jump as u32 as usize;

        s[1..min_jump].fill(b'1');

        let mut count = 0;

        for &c in &s[..=max_jump - min_jump] {
            count += u32::from(b'1' - c);
        }

        let s = Cell::from_mut(s.as_mut_slice()).as_slice_of_cells();

        for ((old, new), target) in s.iter().zip(&s[max_jump - min_jump + 1..]).zip(&s[max_jump + 1..]) {
            count = count.wrapping_add(i32::from(old.get().wrapping_sub(new.get()) as i8) as u32);

            if target.get() == b'0' {
                target.set(b'1' - u8::from(count != 0));
            }
        }

        s.last().unwrap().get() == b'0'
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        Self::can_reach(s, min_jump, max_jump)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
