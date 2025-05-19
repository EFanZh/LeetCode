pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(position: &[i32], mut m: u32, middle: u32) -> bool {
        let mut iter = position.iter().map(|&x| x as u32);
        let mut prev = iter.next().unwrap();

        m -= 1;

        for x in iter {
            if x - prev >= middle {
                m -= 1;

                if m == 0 {
                    return true;
                }

                prev = x;
            }
        }

        false
    }

    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut position = position;
        let position = position.as_mut_slice();
        let m = m as u32;

        assert!(position.len() > 1);

        position.sort_unstable_by_key(|&x| x as u32);

        let mut low = 2;
        let mut high = (*position.last().unwrap() - *position.first().unwrap()) as u32 / (m - 1) + 1;

        while low < high {
            let middle = u32::midpoint(low, high);

            if Self::check(position, m, middle) {
                low = middle + 1;
            } else {
                high = middle;
            }
        }

        (low - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        Self::max_distance(position, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
