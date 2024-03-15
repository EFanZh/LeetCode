pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut counts = [0_u16; 1000];
        let mut prev = 0;

        for num in nums {
            if prev == key {
                counts[num as u32 as usize - 1] += 1;
            }

            prev = num;
        }

        (1..).zip(&counts).max_by_key(|&(_, &count)| count).unwrap().0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        Self::most_frequent(nums, key)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
