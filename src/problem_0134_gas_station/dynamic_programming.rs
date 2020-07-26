pub struct Solution {}

use std::mem;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut cache = vec![0; n];
        let mut temp_cache = vec![0; n];

        for _ in 0..n {
            for (start, (g, c)) in gas.iter().zip(&cost).enumerate() {
                temp_cache[start] = (cache[(start + 1) % n] + c - g).max(0);
            }

            mem::swap(&mut cache, &mut temp_cache);
        }

        cache
            .into_iter()
            .enumerate()
            .find_map(|(i, x)| if x == 0 { Some(i as _) } else { None })
            .unwrap_or(-1)
    }
}

impl super::Solution for Solution {
    fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        Self::can_complete_circuit(gas, cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
