pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU16;

impl Solution {
    fn helper(floor: &mut [bool], mut num_carpets: u16, carpet_len: NonZeroU16) -> u16 {
        const CACHE_CAPACITY: usize = 1 << 10;
        const CACHE_MASK: usize = CACHE_CAPACITY - 1;

        let Some(first_white) = floor.iter().position(|&is_white| is_white) else {
            return 0;
        };

        let Some(last_white) = floor[first_white + 1..].iter().rposition(|&is_white| is_white) else {
            return 0;
        };

        let floor = &mut floor[first_white..first_white + last_white + 2];

        let count_white = |s: &[bool]| s.iter().fold(0, |count, &is_white| count + u16::from(is_white));

        let Some(total_carpet_length) = carpet_len.get().checked_mul(num_carpets) else {
            return 0;
        };

        if total_carpet_length > floor.len() as _ {
            return 0;
        }

        if carpet_len.get() == 1 {
            return count_white(floor).saturating_sub(total_carpet_length);
        }

        let mut retained = 0;
        let mut prev_is_white = false;
        let mut length = 0;
        let mut i = 0;

        while let Some(&is_white) = floor.get(i) {
            if is_white == prev_is_white {
                length += 1;

                if prev_is_white && length as u16 == carpet_len.get() {
                    num_carpets -= 1;

                    if num_carpets == 0 {
                        return count_white(&floor[..retained]) + count_white(&floor[i + 1..]);
                    }

                    length = 0;
                }
            } else {
                floor.copy_within(i - length..i, retained);
                retained += length;
                prev_is_white = is_white;
                length = 1;
            }

            i += 1;
        }

        floor.copy_within(i - length.., retained);
        retained += length;

        let floor = &floor[..retained];
        let white_count = count_white(&floor[..usize::from(carpet_len.get()) - 1]);

        let iter = floor
            .iter()
            .zip(&floor[usize::from(carpet_len.get()) - 1..])
            .enumerate();

        let mut cache = [0_u16; CACHE_CAPACITY];
        let cache_size = floor.len() - usize::from(carpet_len.get()) + 1;

        for _ in 1..=num_carpets {
            let mut white_count = white_count;

            iter.clone().for_each(|(i, (&old, &new))| {
                white_count += u16::from(new);

                cache[i & CACHE_MASK] = white_count + cache[(i + usize::from(carpet_len.get())) & CACHE_MASK];

                white_count -= u16::from(old);
            });

            let mut prev_count = 0;

            cache.iter_mut().take(cache_size).rev().for_each(|count| {
                prev_count = prev_count.max(*count);
                *count = prev_count;
            });
        }

        count_white(floor) - cache[0]
    }

    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        i32::from(Self::helper(
            &mut floor.into_bytes().into_iter().map(|c| c != b'0').collect::<Vec<_>>(),
            num_carpets as _,
            NonZeroU16::new(carpet_len as _).unwrap(),
        ))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        Self::minimum_white_tiles(floor, num_carpets, carpet_len)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
