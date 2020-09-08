pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // https://leetcode.com/problems/gas-station/discuss/42572/Proof-of-%22if-total-gas-is-greater-than-total-cost-there-is-a-solution%22.-C%2B%2B.

        let n = gas.len();
        let mut remaining_gas = 0;
        let mut min_remaining_gas = 0;
        let mut min_index = 0;

        for (i, (g, c)) in gas.into_iter().zip(cost).enumerate() {
            remaining_gas += g - c;

            if remaining_gas < min_remaining_gas {
                min_remaining_gas = remaining_gas;
                min_index = i;
            }
        }

        if remaining_gas < 0 {
            -1
        } else {
            ((min_index + 1) % n) as _
        }
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
