pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct PowerCalculator {
    cache: Vec<u32>,
}

impl PowerCalculator {
    fn get_odd(&mut self, value: u32) -> u32 {
        let index = (value as usize - 1) / 2;

        if let Some(&candidate) = self.cache.get(index) {
            if candidate != u32::MAX {
                return candidate;
            }
        } else {
            self.cache.resize(index + 1, u32::MAX);
        }

        let result = if value == 1 { 0 } else { self.get(value * 3 + 1) + 1 };

        self.cache[index] = result;

        result
    }

    fn get(&mut self, value: u32) -> u32 {
        let trailing_zeros = value.trailing_zeros();

        self.get_odd(value >> trailing_zeros) + trailing_zeros
    }
}

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let (lo, hi, k) = (lo as u32, hi as u32, k as usize);
        let mut calculator = PowerCalculator::default();
        let mut elements = (lo..=hi).rev().map(|i| (calculator.get(i), i)).collect::<Vec<_>>();

        elements.select_nth_unstable(k - 1).1.1 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        Self::get_kth(lo, hi, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
