pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut iter = nums.into_iter().map(i32::cast_unsigned);
        let mut max_0 = 0;
        let mut max_1;
        let mut max_2;

        'outer: loop {
            if let Some(num) = iter.next() {
                match num % 3 {
                    0 => max_0 += num,
                    1 => {
                        max_1 = max_0 + num;

                        loop {
                            if let Some(num) = iter.next() {
                                match num % 3 {
                                    0 => {
                                        max_0 += num;
                                        max_1 += num;
                                    }
                                    1 => {
                                        max_2 = max_1 + num;
                                        max_1 = max_1.max(max_0 + num);

                                        break 'outer;
                                    }
                                    _ => {
                                        max_2 = max_0 + num;
                                        max_0 = max_0.max(max_1 + num);

                                        break 'outer;
                                    }
                                }
                            } else {
                                return max_0 as _;
                            }
                        }
                    }
                    _ => {
                        max_2 = max_0 + num;

                        loop {
                            if let Some(num) = iter.next() {
                                match num % 3 {
                                    0 => {
                                        max_0 += num;
                                        max_2 += num;
                                    }
                                    1 => {
                                        max_1 = max_0 + num;
                                        max_0 = max_0.max(max_2 + num);

                                        break 'outer;
                                    }
                                    _ => {
                                        max_1 = max_2 + num;
                                        max_2 = max_2.max(max_0 + num);

                                        break 'outer;
                                    }
                                }
                            } else {
                                return max_0 as _;
                            }
                        }
                    }
                }
            } else {
                return max_0 as _;
            }
        }

        for num in iter {
            match num % 3 {
                0 => {
                    max_0 += num;
                    max_1 += num;
                    max_2 += num;
                }
                1 => {
                    let new_max_0 = max_0.max(max_2 + num);
                    let new_max_1 = max_1.max(max_0 + num);
                    let new_max_2 = max_2.max(max_1 + num);

                    max_0 = new_max_0;
                    max_1 = new_max_1;
                    max_2 = new_max_2;
                }
                _ => {
                    let new_max_0 = max_0.max(max_1 + num);
                    let new_max_1 = max_1.max(max_2 + num);
                    let new_max_2 = max_2.max(max_0 + num);

                    max_0 = new_max_0;
                    max_1 = new_max_1;
                    max_2 = new_max_2;
                }
            }
        }

        max_0 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        Self::max_sum_div_three(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
