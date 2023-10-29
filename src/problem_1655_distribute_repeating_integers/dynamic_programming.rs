pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn count_nums(nums: Vec<i32>) -> Box<[u32]> {
        let mut counts = [0; 1000];
        let mut unique_numbers = Vec::with_capacity(50);

        for num in nums {
            let index = num as u32 as usize - 1;
            let count = &mut counts[index];

            if *count == 0 {
                unique_numbers.push(index as _);
            }

            *count += 1;
        }

        let mut unique_numbers = unique_numbers.into_boxed_slice();

        for slot in &mut *unique_numbers {
            *slot = counts[*slot as usize];
        }

        unique_numbers
    }

    fn get_quantity_sums(value: Vec<i32>) -> ([u32; 1024], usize) {
        let n = value.len();
        let mut result = [0_u32; 1024];

        assert!(n <= 10);

        for bits in 1..(1 << n) {
            result[bits] = result[bits & (bits - 1)] + value[bits.trailing_zeros() as usize] as u32;
        }

        (result, n)
    }

    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        let counts = Self::count_nums(nums);
        let (quantity_sums, n) = Self::get_quantity_sums(quantity);
        let cache_size = 1 << n;
        let mut buffer = [false; 2048];
        let (mut cache, mut temp) = buffer[..cache_size * 2].split_at_mut(cache_size);

        cache[0] = true;

        for &count in &*counts {
            for (target_bits, target) in (0_u16..).zip(&mut *temp) {
                let mut selected_bits = target_bits;

                // See <https://cp-algorithms.com/algebra/all-submasks.html>.

                *target = loop {
                    if quantity_sums[usize::from(selected_bits)] <= count
                        && cache[usize::from(target_bits ^ selected_bits)]
                    {
                        break true;
                    }

                    if selected_bits == 0 {
                        break false;
                    }

                    selected_bits = (selected_bits - 1) & target_bits;
                };
            }

            mem::swap(&mut cache, &mut temp);
        }

        *cache.last().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        Self::can_distribute(nums, quantity)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
