pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut index_of_1 = 0;
        let mut index_of_n = 0;

        (0..).zip(nums).for_each(|(i, num)| {
            if num == 1 {
                index_of_1 = i;
            } else if num == n {
                index_of_n = i;
            }
        });

        index_of_1 + (n - 1 - index_of_n) - i32::from(index_of_n < index_of_1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        Self::semi_ordered_permutation(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
