pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut result = 0;
        let mut poisoned = i32::min_value();
        let mut healed = i32::min_value();

        for time_point in time_series {
            if time_point > healed {
                result += healed - poisoned;
                poisoned = time_point;
            }

            healed = time_point + duration;
        }

        result + healed - poisoned
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        Self::find_poisoned_duration(time_series, duration)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
