pub mod find_median;

pub trait Solution {
    fn wiggle_sort(nums: &mut Vec<i32>);
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            &[1, 5, 1, 1, 6, 4] as &[_],
            &[1, 3, 2, 2, 3, 1],
            &[4, 5, 5, 6],
            &[1, 3, 2, 2, 3, 1],
            &[2, 3, 5, 7, 11, 13, 17, 19, 23],
            &[3],
        ];

        for nums in test_cases.iter().copied() {
            let mut nums_2 = nums.to_vec();

            S::wiggle_sort(&mut nums_2);

            if nums.len() > 1 {
                let mut direction = false;

                for (lhs, rhs) in nums_2.iter().zip(&nums_2[1..]) {
                    if direction {
                        assert!(lhs > rhs);
                    } else {
                        assert!(lhs < rhs);
                    }

                    direction = !direction;
                }
            }

            nums_2.sort_unstable();

            assert_eq!(nums_2, test_utilities::unstable_sorted(nums.iter().copied()));
        }
    }
}
