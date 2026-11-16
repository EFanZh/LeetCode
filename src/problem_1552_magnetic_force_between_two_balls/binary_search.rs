pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(position: &[u32], mut m: u32, middle: u32) -> bool {
        let mut iter = position.iter().copied();
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
        let mut position = position.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let position = position.as_mut_slice();
        let m = m.cast_unsigned();

        assert!(position.len() > 1);

        position.sort_unstable();

        let mut low = 2;
        let mut high = (*position.last().unwrap() - position.first().unwrap()) / (m - 1) + 1;

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
