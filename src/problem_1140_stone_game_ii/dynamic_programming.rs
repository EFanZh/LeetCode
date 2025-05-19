pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut sums = piles;
        let mut sum = 0;

        for value in &mut sums {
            sum += *value;
            *value = sum;
        }

        let n = sums.len();
        let columns = usize::midpoint(n, 2);
        let mut cache = vec![0; columns * (n + 1)];

        for length in 1..=n {
            let start = n - length;

            for m in 1..=usize::midpoint(start, 2) {
                let mut max_diff = i32::MIN;

                for selection in 1..=length.min(m * 2) {
                    let score = sums[start + selection - 1] - sums.get(start.wrapping_sub(1)).copied().unwrap_or(0);

                    max_diff = max_diff.max(score - cache[columns * (length - selection) + m.max(selection) - 1]);
                }

                cache[columns * length + m - 1] = max_diff;
            }
        }

        let diff = cache[columns * n];

        i32::midpoint(sum, diff)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn stone_game_ii(piles: Vec<i32>) -> i32 {
        Self::stone_game_ii(piles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
