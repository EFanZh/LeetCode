pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn update_candidate(upper_bound: i32, middle: i32, target: &mut i32) {
        if middle >= upper_bound {
            *target += middle - (upper_bound - 1);
        }
    }

    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut candidate_1 = 0;
        let mut candidate_2 = 0;
        let mut iter = nums.into_iter();
        let mut left = i32::MAX;
        let mut middle = iter.next().unwrap();

        loop {
            if let Some(right) = iter.next() {
                Self::update_candidate(left.min(right), middle, &mut candidate_1);

                left = middle;
                middle = right;
            } else {
                Self::update_candidate(left, middle, &mut candidate_1);

                break;
            }

            if let Some(right) = iter.next() {
                Self::update_candidate(left.min(right), middle, &mut candidate_2);

                left = middle;
                middle = right;
            } else {
                Self::update_candidate(left, middle, &mut candidate_2);

                break;
            }
        }

        candidate_1.min(candidate_2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        Self::moves_to_make_zigzag(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
