pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let n = n as u32 as usize;
        let max_sum = max_sum as u32;
        let mut banned_map = vec![false; n].into_boxed_slice();

        for num in banned {
            if let Some(is_banned) = banned_map.get_mut(num as u32 as usize - 1) {
                *is_banned = true;
            }
        }

        let mut sum = 0;
        let mut count = 0;

        (1..).zip(&*banned_map).try_for_each(|(num, &is_banned)| {
            if !is_banned {
                let candidate = sum + num;

                if candidate <= max_sum {
                    sum = candidate;
                    count += 1;

                    if candidate == max_sum {
                        return None;
                    }
                } else {
                    return None;
                }
            }

            Some(())
        });

        count
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        Self::max_count(banned, n, max_sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
