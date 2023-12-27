pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut states = [0_u8; 100];

        for num in nums1 {
            states[num as u32 as usize - 1] = 1;
        }

        let mut result = Vec::new();

        for num in nums2 {
            let state = &mut states[num as u32 as usize - 1];

            if *state == 1 {
                result.push(num);
            }

            *state |= 2;
        }

        for num in nums3 {
            let state = &mut states[num as u32 as usize - 1];

            if *state == 1 || *state == 2 {
                result.push(num);
            }

            *state |= 4;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        Self::two_out_of_three(nums1, nums2, nums3)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
