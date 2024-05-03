pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut result = vec![0; nums1.len()];
        let mut nums2 = result.iter_mut().zip(nums2).collect::<Vec<_>>();

        nums1.sort_unstable();
        nums2.sort_unstable_by_key(|&(_, value)| value);

        let mut stack = Vec::new();
        let mut extra = Vec::new();
        let mut iter_1 = nums1.into_iter();
        let mut iter_2 = nums2.into_iter();
        let mut value_1 = iter_1.next().unwrap();
        let (mut slot, mut value_2) = iter_2.next().unwrap();

        loop {
            if value_2 < value_1 {
                stack.push(slot);

                if let Some((next_slot, next_value_2)) = iter_2.next() {
                    slot = next_slot;
                    value_2 = next_value_2;
                } else {
                    *stack.pop().unwrap() = value_1;

                    for value in iter_1 {
                        *stack.pop().unwrap() = value;
                    }

                    break;
                }
            } else {
                if let Some(slot) = stack.pop() {
                    *slot = value_1;
                } else {
                    extra.push(value_1);
                }

                if let Some(next_value_1) = iter_1.next() {
                    value_1 = next_value_1;
                } else {
                    *slot = extra.pop().unwrap();

                    for (slot, _) in iter_2 {
                        *slot = extra.pop().unwrap();
                    }

                    break;
                }
            }
        }

        for (slot, value) in stack.into_iter().zip(extra) {
            *slot = value;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        Self::advantage_count(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
