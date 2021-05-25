pub struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        nums.split_first().map_or(true, |(&first, rest)| {
            let mut prev = first;
            let mut iter = rest.iter().copied();

            while let Some(mut num) = iter.next() {
                if num < prev {
                    loop {
                        if num > first {
                            return false;
                        }

                        prev = num;

                        if let Some(next) = iter.next() {
                            if next < prev {
                                return false;
                            }

                            num = next;
                        } else {
                            return true;
                        }
                    }
                }

                prev = num;
            }

            true
        })
    }
}

impl super::Solution for Solution {
    fn check(nums: Vec<i32>) -> bool {
        Self::check(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
