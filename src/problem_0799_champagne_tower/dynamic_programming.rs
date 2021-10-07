pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_overflow(incoming: f64) -> f64 {
        if incoming <= 1.0 {
            0.0
        } else {
            incoming - 1.0
        }
    }

    fn update_cache(cache: &mut [f64], end: usize) {
        for i in (0..end).rev() {
            let left = cache.get(i.wrapping_sub(1)).copied();
            let right = cache[i];
            let incoming = left.map_or(right, |left| left + right) / 2.0;

            cache[i] = Self::get_overflow(incoming);
        }
    }

    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let (poured, query_row, query_glass) = (poured as u32, query_row as usize, query_glass as usize);

        if poured == 0 {
            0.0
        } else if query_row == 0 {
            1.0
        } else {
            let mut cache = vec![0.0; (query_row + 1) / 2];

            cache[0] = f64::from(poured - 1);

            let pairs = (query_row + 1) / 2;

            for i in 1..pairs {
                Self::update_cache(&mut cache, i);

                cache[i] = Self::get_overflow(cache[i - 1]);

                Self::update_cache(&mut cache, i);
            }

            if query_row % 2 == 0 {
                Self::update_cache(&mut cache, pairs);
            }

            let query_glass = query_glass.min(query_row - query_glass);
            let left = cache.get(query_glass.wrapping_sub(1)).copied().unwrap_or(0.0);
            let right = cache[query_glass.min(cache.len() - 1)];
            let incoming = (left + right) / 2.0;

            incoming.min(1.0)
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        Self::champagne_tower(poured, query_row, query_glass)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
