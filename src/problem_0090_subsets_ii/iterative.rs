pub struct Solution;

impl Solution {
    fn extend_result(result: &mut Vec<Vec<i32>>, num: i32, extra: u32) {
        for i in 0..result.len() {
            let mut subset = result[i].clone();

            for _ in 0..extra {
                subset.push(num);
                result.push(subset.clone());
            }

            subset.push(num);
            result.push(subset);
        }
    }

    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![Vec::new()];

        nums.sort_unstable();

        let mut iter = nums.into_iter();

        if let Some(mut first) = iter.next() {
            let mut extra = 0;

            loop {
                if let Some(num) = iter.next() {
                    if num == first {
                        extra += 1;
                    } else {
                        Self::extend_result(&mut result, first, extra);

                        first = num;
                        extra = 0;
                    }
                } else {
                    Self::extend_result(&mut result, first, extra);

                    break;
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::subsets_with_dup(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
