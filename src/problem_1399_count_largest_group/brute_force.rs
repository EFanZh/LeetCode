pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let n = (n + 1) as usize;
        let mut counts = [0_u16; 37];

        for mut x in 1..n {
            let mut sum = 0;

            loop {
                sum += x % 10;
                x /= 10;

                if x == 0 {
                    break;
                }
            }

            counts[sum] += 1;
        }

        let mut max_group_size = 0;
        let mut max_group_count = 0;

        for count in counts {
            match count.cmp(&max_group_size) {
                Ordering::Less => {}
                Ordering::Equal => max_group_count += 1,
                Ordering::Greater => {
                    max_group_size = count;
                    max_group_count = 1;
                }
            }
        }

        max_group_count
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_largest_group(n: i32) -> i32 {
        Self::count_largest_group(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
