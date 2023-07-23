pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn normalize(mut x: u32) -> u32 {
        const MODULUS: u32 = 1_000_000_007;

        x = x.checked_sub(MODULUS).unwrap_or(x);

        x.checked_sub(MODULUS).unwrap_or(x)
    }

    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        // Sort locations.

        let mut locations = locations.into_boxed_slice();

        let start_location = locations[start as u32 as usize];
        let finish_location = locations[finish as u32 as usize];

        locations.sort_unstable();

        let start = locations.binary_search(&start_location).unwrap();
        let finish = locations.binary_search(&finish_location).unwrap();

        // Dynamic programming.

        let n = locations.len();
        let fuel = fuel as u32 as usize;
        let mut cache = vec![(0_u32, 0_u32); n * (fuel + 1)].into_boxed_slice();
        let index = |fuel, i| n.wrapping_mul(fuel).wrapping_add(i);

        for fuel in 1..=fuel {
            for (i, &location) in locations.iter().enumerate() {
                let location = location as u32 as usize;

                // Move left.

                let left = i.wrapping_sub(1);

                let left_count = locations
                    .get(left)
                    .and_then(|&left_location| {
                        let next =
                            cache.get(index(fuel.wrapping_sub(location - left_location as u32 as usize), left))?;

                        Some(Self::normalize(next.0 * 2 + u32::from(left == finish) + next.1))
                    })
                    .unwrap_or(0);

                // Move right.

                let right = i + 1;

                let right_count = locations
                    .get(right)
                    .and_then(|&right_location| {
                        let next = cache.get(index(
                            fuel.wrapping_sub(right_location as u32 as usize - location),
                            right,
                        ))?;

                        Some(Self::normalize(next.0 + u32::from(right == finish) + next.1 * 2))
                    })
                    .unwrap_or(0);

                cache[index(fuel, i)] = (left_count, right_count);
            }
        }

        let count = cache[index(fuel, start)];

        Self::normalize(count.0 + u32::from(start == finish) + count.1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        Self::count_routes(locations, start, finish, fuel)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
