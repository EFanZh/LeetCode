pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let m = m as u32 as usize;
        let n = n as u32 as usize;
        let mut cache = vec![0_u64; n * m].into_boxed_slice();

        for price in prices {
            let [height, width, price] = price.try_into().ok().unwrap();

            cache[n * (height as u32 as usize - 1) + width as u32 as usize - 1] = u64::from(price as u32);
        }

        let mut row_start = 0;

        for y in 0..m {
            for x in 0..n {
                let index = row_start + x;
                let mut max_price = cache[index];

                iter::zip(cache[x..].iter().step_by(n), cache[x..index].iter().step_by(n).rev())
                    .take(y.div_ceil(2))
                    .for_each(|(left, right)| max_price = max_price.max(left + right));

                iter::zip(&cache[row_start..], cache[..index].iter().rev())
                    .take(x.div_ceil(2))
                    .for_each(|(left, right)| max_price = max_price.max(left + right));

                cache[index] = max_price;
            }

            row_start += n;
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        Self::selling_wood(m, n, prices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
