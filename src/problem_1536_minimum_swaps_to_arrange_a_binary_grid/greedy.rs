pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let mut values = grid
            .into_iter()
            .map(|row| {
                let mut iter = row.iter().copied();
                let mut count = 0_u8;

                while iter.next_back() == Some(0) {
                    count += 1;
                }

                count
            })
            .collect::<Box<_>>();

        let n = values.len();
        let mut min = n as u8;
        let mut result = 0;

        for i in 0..n {
            let window = &mut values[i..];

            min -= 1;

            if let Some(j) = window.iter().position(|&value| value >= min) {
                result += j;
                window[..=j].rotate_right(1);
            } else {
                return -1;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        Self::min_swaps(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
