pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut states = [false; 500];

        for num in nums {
            let state = &mut states[num as u32 as usize - 1];

            *state = !*state;
        }

        for state in states {
            if state {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn divide_array(nums: Vec<i32>) -> bool {
        Self::divide_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
