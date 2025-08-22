pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        let mut coins = coins.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let target = target.cast_unsigned();

        coins.sort_unstable();

        let mut reachable = 0;
        let mut result = 0;

        for coin in coins {
            while reachable + 1 < coin {
                result += 1;
                reachable = reachable * 2 + 1;

                if reachable >= target {
                    return result;
                }
            }

            reachable += coin;

            if reachable >= target {
                return result;
            }
        }

        loop {
            result += 1;
            reachable = reachable * 2 + 1;

            if reachable >= target {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        Self::minimum_added_coins(coins, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
