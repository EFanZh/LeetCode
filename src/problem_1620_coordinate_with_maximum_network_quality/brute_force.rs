pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        const N: usize = 51;

        let radius = radius as u16;
        let radius_squared = radius * radius;

        #[expect(clippy::large_stack_arrays, reason = "by-design")]
        let mut multipliers = [0_f64; N * N];

        let radius_plus_1 = radius + 1;

        for x in 0..radius_plus_1 {
            let x_squared = x * x;

            for y in x..radius_plus_1 {
                if x_squared + y * y <= radius_squared {
                    multipliers[usize::from(radius_plus_1) * usize::from(x) + usize::from(y)] =
                        1.0 / (1.0 + f64::from(x).hypot(f64::from(y)));
                }
            }
        }

        let mut quality_map = [0_u8; N * N];
        let mut min_x = u16::MAX;
        let mut max_x = 0;
        let mut min_y = u16::MAX;
        let mut max_y = 0;

        for tower in towers {
            let [x, y, q] = tower.try_into().ok().unwrap();
            let x = x as u16;
            let y = y as u16;
            let q = f64::from(q);

            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);

            for x_2 in x.saturating_sub(radius)..(x + radius_plus_1).min(N as _) {
                let dx = x_2.abs_diff(x);

                for y_2 in y.saturating_sub(radius)..(y + radius_plus_1).min(N as _) {
                    let dy = y_2.abs_diff(y);
                    let (dx, dy) = if dy < dx { (dy, dx) } else { (dx, dy) };

                    quality_map[N * usize::from(x_2) + usize::from(y_2)] +=
                        (q * multipliers[usize::from(radius_plus_1) * usize::from(dx) + usize::from(dy)]) as u8;
                }
            }
        }

        let mut max_quality = 0;
        let mut result = [0, 0];

        for (x, row) in quality_map
            .chunks_exact(N)
            .enumerate()
            .take(usize::from(max_x) + 1)
            .skip(usize::from(min_x))
        {
            for (y, &quality) in row
                .iter()
                .enumerate()
                .take(usize::from(max_y) + 1)
                .skip(usize::from(min_y))
            {
                if quality > max_quality {
                    max_quality = quality;

                    result = [x as _, y as _];
                }
            }
        }

        Vec::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        Self::best_coordinate(towers, radius)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
