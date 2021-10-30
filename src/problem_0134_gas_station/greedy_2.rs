pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut gas = gas;

        for (g, c) in gas.iter_mut().zip(cost) {
            *g -= c;
        }

        let n = gas.len();
        let mut start = n;
        let mut length = 0;
        let mut sum = 0;

        'outer: loop {
            sum += gas[start + length - n];
            length += 1;

            if length == n {
                break;
            }

            while sum < 0 {
                start -= 1;
                length += 1;
                sum += gas[start];

                if length == n {
                    break 'outer;
                }
            }
        }

        if sum < 0 {
            -1
        } else {
            (start % n) as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
