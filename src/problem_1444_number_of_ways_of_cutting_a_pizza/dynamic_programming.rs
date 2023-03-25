pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn count_apples(pizza: &[String], rows: usize, columns: usize) -> Box<[u32]> {
        let mut result = vec![0; columns * rows].into_boxed_slice();
        let mut iter = result.chunks_exact_mut(columns).zip(pizza);
        let (mut prev_targets, row) = iter.next_back().unwrap();

        let mut sum = 0;

        for (target, cell) in prev_targets.iter_mut().zip(row.bytes()).rev() {
            sum += u32::from(cell == b'A');
            *target = sum;
        }

        for (targets, row) in iter.rev() {
            sum = 0;

            for (target, (bottom, cell)) in targets.iter_mut().zip(prev_targets.iter_mut().zip(row.bytes())).rev() {
                sum += u32::from(cell == b'A');
                *target = sum + *bottom;
            }

            prev_targets = targets;
        }

        result
    }

    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        const MODULUS: u32 = 1_000_000_007;

        let k = k as u32;
        let rows = pizza.len();
        let columns = pizza.first().map_or(0, String::len);
        let apples = Self::count_apples(&pizza, rows, columns);
        let mut cache = apples.iter().map(|&count| u32::from(count != 0)).collect::<Box<_>>();
        let mut temp = vec![0; columns * rows].into_boxed_slice();

        for _ in 1..k {
            for (y, (targets, counts)) in temp
                .chunks_exact_mut(columns)
                .zip(apples.chunks_exact(columns))
                .enumerate()
            {
                for (x, (target, &total_apples)) in targets.iter_mut().zip(counts).enumerate() {
                    let mut ways = 0;

                    // Horizontal cuts.

                    for (&bottom_apples, bottom_ways) in
                        apples.iter().zip(&*cache).skip(columns * (y + 1) + x).step_by(columns)
                    {
                        if total_apples != bottom_apples {
                            ways += bottom_ways;

                            if ways >= MODULUS {
                                ways -= MODULUS;
                            }
                        }
                    }

                    // Vertical cuts.

                    for (&right_apples, right_ways) in apples
                        .iter()
                        .zip(&*cache)
                        .skip(columns * y + x + 1)
                        .take(columns - (x + 1))
                    {
                        if total_apples != right_apples {
                            ways += right_ways;

                            if ways >= MODULUS {
                                ways -= MODULUS;
                            }
                        }
                    }

                    // Save result.

                    *target = ways;
                }
            }

            mem::swap(&mut cache, &mut temp);
        }

        cache[0] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ways(pizza: Vec<String>, k: i32) -> i32 {
        Self::ways(pizza, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
