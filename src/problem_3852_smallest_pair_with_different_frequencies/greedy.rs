pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = [0_u32; 100];

        for num in nums {
            counts[num.cast_unsigned() as usize - 1] += 1;
        }

        let mut iter = (1..).zip(&counts).filter(|&(_, &count)| count != 0);

        iter.next()
            .and_then(|(x, &x_frequency)| {
                iter.find_map(|(y, &y_frequency)| (x_frequency != y_frequency).then_some([x, y]))
            })
            .unwrap_or([-1, -1])
            .into()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        Self::min_distinct_freq_pair(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
