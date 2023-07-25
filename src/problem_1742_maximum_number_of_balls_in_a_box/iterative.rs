pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn sum_digits(cache: &mut [u8], x: u32) -> u8 {
        if x < 10 {
            x as u8
        } else {
            let mut candidate = cache[x as usize];

            if candidate == 0 {
                candidate = (x % 10) as u8 + Self::sum_digits(cache, x / 10);

                cache[x as usize] = candidate;
            }

            candidate
        }
    }

    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let low_limit = low_limit as u32;
        let high_limit = high_limit as u32;
        let mut cache = vec![0; high_limit as usize + 1].into_boxed_slice();
        let mut counts = [0; 46];

        for x in low_limit..=high_limit {
            counts[usize::from(Self::sum_digits(&mut cache, x))] += 1;
        }

        *counts.iter().max().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        Self::count_balls(low_limit, high_limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
