pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let start = nums.len() as i32;
        let mut slow = start;
        let mut fast = start;

        loop {
            slow = nums[(slow - 1) as usize];
            fast = nums[(nums[(fast - 1) as usize] - 1) as usize];

            if slow == fast {
                break;
            }
        }

        slow = start;

        while slow != fast {
            slow = nums[(slow - 1) as usize];
            fast = nums[(fast - 1) as usize];
        }

        slow
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_duplicate(nums: Vec<i32>) -> i32 {
        Self::find_duplicate(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
