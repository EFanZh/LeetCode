pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let [energy_drink_a, energy_drink_b] =
            [energy_drink_a, energy_drink_b].map(|v| v.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>());

        let (max_boost_1, max_boost_2, _) =
            energy_drink_a
                .iter()
                .zip(&energy_drink_b)
                .fold((0, 0, 0), |cache, (&boost_1, &boost_2)| {
                    let [boost_1, boost_2] = [boost_1, boost_2].map(u64::from);

                    (
                        (cache.0).max(cache.2) + boost_1,
                        (cache.1).max(cache.2) + boost_2,
                        cache.0.max(cache.1),
                    )
                });

        max_boost_1.max(max_boost_2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        Self::max_energy_boost(energy_drink_a, energy_drink_b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
