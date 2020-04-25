pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut retained = 0;
        let mut i = 0;

        while let Some(item) = nums.get(i) {
            if *item != val {
                nums.swap(retained, i);
                retained += 1;
            }

            i += 1;
        }

        nums.truncate(retained);

        nums.len() as _
    }
}

impl super::Solution for Solution {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        Self::remove_element(nums, val)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}
