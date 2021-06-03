pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut min_w = m;
        let mut min_h = n;

        for op in ops {
            let [w, h]: [i32; 2] = op.as_slice().try_into().unwrap();

            min_w = min_w.min(w);
            min_h = min_h.min(h);
        }

        min_w * min_h
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        Self::max_count(m, n, ops)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
