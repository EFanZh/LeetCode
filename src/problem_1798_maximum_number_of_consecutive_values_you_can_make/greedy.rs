pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/maximum-number-of-consecutive-values-you-can-make/discuss/1118770/JavaC%2B%2BPython-Accumulate-the-Coins>.

impl Solution {
    pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        let mut coins = coins;

        coins.sort_unstable();

        let mut result = 1;

        for coin in coins {
            if coin > result {
                break;
            }

            result += coin;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        Self::get_maximum_consecutive(coins)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
