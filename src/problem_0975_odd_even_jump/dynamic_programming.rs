pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BTreeMap;

impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut values = BTreeMap::<i32, usize>::new();
        let mut nexts = vec![(0, 0); n];

        for (i, (target, &value)) in nexts.iter_mut().zip(&arr).enumerate().rev() {
            if let Some((_, &index)) = values.range(value..).next() {
                target.0 = index;
            }

            if let Some((_, &index)) = values.range(..=value).next_back() {
                target.1 = index;
            }

            values.insert(value, i);
        }

        let mut cache = vec![(false, false); n];

        cache[n - 1] = (true, true);

        let mut result = 1;

        for (i, (next_odd, next_even)) in nexts[..n - 1].iter().copied().enumerate().rev() {
            let odd_ok = cache[next_odd].1;
            let even_ok = cache[next_even].0;

            cache[i] = (odd_ok, even_ok);

            if odd_ok {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        Self::odd_even_jumps(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
