pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let mut stones = stones;

        stones.sort_unstable();

        let n = stones.len();
        let n_minus_2 = n - 2;
        let mut start = 0;
        let mut low = n as i32;

        for (i, &stone) in stones.iter().enumerate() {
            while stones[start] <= stone - n as i32 {
                start += 1;
            }

            low = low.min(if i - start == n_minus_2 && stone - stones[start] == n_minus_2 as i32 {
                2
            } else {
                (n - 1 - (i - start)) as _
            });
        }

        let high = (stones[n - 1] - stones[1]).max(stones[n - 2] - stones[0]) - n_minus_2 as i32;

        vec![low, high]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        Self::num_moves_stones_ii(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
