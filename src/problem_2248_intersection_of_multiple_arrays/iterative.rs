pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len() as u16;
        let mut counts = [0_u16; 1000];

        for nums in nums {
            for num in nums {
                counts[num as u32 as usize - 1] += 1;
            }
        }

        (1_u32..)
            .zip(&counts)
            .filter_map(|(i, &count)| (count == n).then_some(i as _))
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        Self::intersection(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
