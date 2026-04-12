pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32> {
        const MODULUS: u64 = 1_000_000_007;

        let n = conversions.len() + 1;
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for conversion in conversions {
            let [source, target, factor] = conversion.try_into().ok().unwrap();

            graph[source.cast_unsigned() as usize].push((target.cast_unsigned(), factor.cast_unsigned()));
        }

        let mut stack = Vec::new();
        let mut node = 0;
        let mut value = 1;
        let mut result = vec![0; n];

        loop {
            result[node] = value as _;

            for &(child, factor) in &graph[node] {
                stack.push((child, ((value * u64::from(factor)) % MODULUS) as u32));
            }

            if let Some(next) = stack.pop() {
                node = next.0 as usize;
                value = u64::from(next.1);
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32> {
        Self::base_unit_conversions(conversions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
