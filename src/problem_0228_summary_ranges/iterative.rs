pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut iter = nums.into_iter();

        if let Some(mut first) = iter.next() {
            'outer: loop {
                if let Some(mut last) = iter.next() {
                    if last == first + 1 {
                        loop {
                            if let Some(num) = iter.next() {
                                if num == last + 1 {
                                    last = num;
                                } else {
                                    result.push(format!("{}->{}", first, last));

                                    first = num;

                                    break;
                                }
                            } else {
                                result.push(format!("{}->{}", first, last));

                                break 'outer;
                            }
                        }
                    } else {
                        result.push(first.to_string());

                        first = last;
                    }
                } else {
                    result.push(first.to_string());

                    break;
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        Self::summary_ranges(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
