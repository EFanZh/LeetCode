pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as u32 as usize;
        let mut diffs = vec![0; limit * 2 + 1].into_boxed_slice();
        let (left, right) = nums.split_at(nums.len() / 2);

        for (&x, &y) in left.iter().zip(right.iter().rev()) {
            let (x, y) = (x as u32 as usize, y as u32 as usize);
            let (min, max) = if y < x { (y, x) } else { (x, y) };

            diffs[min] -= 1;
            diffs[min + max - 1] -= 1;
            diffs[min + max] += 1;
            diffs[max + limit] += 1;
        }

        let mut moves = nums.len() as i32;
        let mut result = moves;

        for &diff in &diffs[1..] {
            moves += diff;
            result = result.min(moves);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        Self::min_moves(nums, limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
