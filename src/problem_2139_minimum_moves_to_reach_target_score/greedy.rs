pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut target = target as u32;
        let mut max_doubles = max_doubles as u32;
        let mut result = 0;

        if target > 1 {
            loop {
                if max_doubles == 0 {
                    result += target - 1;

                    break;
                }

                result += (target & 1) + 1;
                target >>= 1;

                if target > 1 {
                    max_doubles -= 1;
                } else {
                    break;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_moves(target: i32, max_doubles: i32) -> i32 {
        Self::min_moves(target, max_doubles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
