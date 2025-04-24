pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let make_iter = |nums: Vec<Vec<i32>>| {
            nums.into_iter()
                .map(|x| <(_, _)>::from(<[_; 2]>::try_from(x.as_slice()).ok().unwrap()))
        };

        let mut result = Vec::with_capacity(nums1.len() + nums2.len());
        let mut add_result = |(id, value)| result.push(vec![id, value]);
        let mut iter_1 = make_iter(nums1);
        let mut iter_2 = make_iter(nums2);
        let mut left = iter_1.next().unwrap();

        'outer: loop {
            if let Some(right) = iter_2.next() {
                loop {
                    if left.0 < right.0 {
                        add_result(left);
                    } else {
                        if left.0 == right.0 {
                            left.1 += right.1;
                        } else {
                            add_result(right);
                        }

                        break;
                    }

                    if let Some(next_left) = iter_1.next() {
                        left = next_left;
                    } else {
                        left = right;

                        break 'outer;
                    }
                }
            } else {
                iter_2 = iter_1;

                break;
            }
        }

        add_result(left);
        iter_2.for_each(add_result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::merge_arrays(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
