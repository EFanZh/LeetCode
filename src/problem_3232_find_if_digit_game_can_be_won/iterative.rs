pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut diff = 0;

        for num in nums {
            if matches!(num, 0..10) {
                diff += num;
            } else {
                diff -= num;
            }
        }

        diff != 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_alice_win(nums: Vec<i32>) -> bool {
        Self::can_alice_win(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
