pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let (plants, capacity_a, capacity_b) = (
            plants.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>(),
            capacity_a as u32,
            capacity_b as u32,
        );

        let (left, right) = plants.split_at(plants.len() / 2);
        let mut alice_available = capacity_a;
        let mut bob_available = capacity_b;
        let mut bob_iter = right.iter();
        let mut result = 0;

        for (&alice_required, &bob_required) in left.iter().zip(bob_iter.by_ref().rev()) {
            if alice_available < alice_required {
                result += 1;
                alice_available = capacity_a;
            }

            alice_available -= alice_required;

            if bob_available < bob_required {
                result += 1;
                bob_available = capacity_b;
            }

            bob_available -= bob_required;
        }

        if let Some(&middle_required) = bob_iter.next() {
            result += u32::from(alice_available.max(bob_available) < middle_required);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        Self::minimum_refill(plants, capacity_a, capacity_b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
