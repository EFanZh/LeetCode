pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut iter = nums.into_iter().map(|num| num as u32);
        let mut cache = (0, 0, 0);

        'outer: loop {
            if let Some(num) = iter.next() {
                match num % 3 {
                    0 => cache.0 += num,
                    1 => {
                        cache.1 = cache.0 + num;

                        loop {
                            if let Some(num) = iter.next() {
                                match num % 3 {
                                    0 => {
                                        cache.0 += num;
                                        cache.1 += num;
                                    }
                                    1 => {
                                        cache.2 = cache.1 + num;
                                        cache.1 = cache.1.max(cache.0 + num);

                                        break 'outer;
                                    }
                                    _ => {
                                        cache.2 = cache.0 + num;
                                        cache.0 = cache.0.max(cache.1 + num);

                                        break 'outer;
                                    }
                                }
                            } else {
                                return cache.0 as _;
                            }
                        }
                    }
                    _ => {
                        cache.2 = cache.0 + num;

                        loop {
                            if let Some(num) = iter.next() {
                                match num % 3 {
                                    0 => {
                                        cache.0 += num;
                                        cache.2 += num;
                                    }
                                    1 => {
                                        cache.1 = cache.0 + num;
                                        cache.0 = cache.0.max(cache.2 + num);

                                        break 'outer;
                                    }
                                    _ => {
                                        cache.1 = cache.2 + num;
                                        cache.2 = cache.2.max(cache.0 + num);

                                        break 'outer;
                                    }
                                }
                            } else {
                                return cache.0 as _;
                            }
                        }
                    }
                }
            } else {
                return cache.0 as _;
            }
        }

        for num in iter {
            cache = match num % 3 {
                0 => (cache.0 + num, cache.1 + num, cache.2 + num),
                1 => (
                    cache.0.max(cache.2 + num),
                    cache.1.max(cache.0 + num),
                    cache.2.max(cache.1 + num),
                ),
                _ => (
                    cache.0.max(cache.1 + num),
                    cache.1.max(cache.2 + num),
                    cache.2.max(cache.0 + num),
                ),
            };
        }

        cache.0 as _
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
