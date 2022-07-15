pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut target = target;
        let mut result = 0;

        while target > start_value {
            if target % 2 == 0 {
                target /= 2;
                result += 1;
            } else {
                target = (target + 1) / 2;
                result += 2;
            }
        }

        result + (start_value - target)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn broken_calc(start_value: i32, target: i32) -> i32 {
        Self::broken_calc(start_value, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
