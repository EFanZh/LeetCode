pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::num::NonZero;

trait Map<'a> {
    fn inc(&mut self, value: &'a [u8]);
    fn dec(&mut self, value: &'a [u8]) -> bool;
}

impl<'a, const N: usize> Map<'a> for [u32; N] {
    fn inc(&mut self, value: &'a [u8]) {
        self[value.iter().fold(0_usize, |key, c| key * 26 + usize::from(c - b'a'))] += 1;
    }

    fn dec(&mut self, value: &'a [u8]) -> bool {
        let count = &mut self[value.iter().fold(0_usize, |key, c| key * 26 + usize::from(c - b'a'))];
        let result = *count != 0;

        if result {
            *count -= 1;
        }

        result
    }
}

trait Encoding<'a>: Eq + Hash {
    fn encode(value: &'a [u8]) -> Self;
}

impl<'a, T> Map<'a> for HashMap<T, u32>
where
    T: Encoding<'a>,
{
    fn inc(&mut self, value: &'a [u8]) {
        match self.entry(T::encode(value)) {
            Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(0);
            }
        }
    }

    fn dec(&mut self, value: &'a [u8]) -> bool {
        let Entry::Occupied(occupied_entry) = self.entry(T::encode(value)) else {
            return false;
        };

        if *occupied_entry.get() == 0 {
            occupied_entry.remove();
        } else {
            *occupied_entry.into_mut() -= 1;
        }

        true
    }
}

impl Encoding<'_> for u32 {
    fn encode(value: &[u8]) -> Self {
        value.iter().fold(0, |key, &c| key * 26 + Self::from(c - b'a'))
    }
}

impl Encoding<'_> for u64 {
    fn encode(value: &[u8]) -> Self {
        value.iter().fold(0, |key, &c| key * 26 + Self::from(c - b'a'))
    }
}

impl Encoding<'_> for u128 {
    fn encode(value: &[u8]) -> Self {
        value.iter().fold(0, |key, &c| key * 26 + Self::from(c - b'a'))
    }
}

impl<'a> Encoding<'a> for &'a [u8] {
    fn encode(value: &'a [u8]) -> Self {
        value
    }
}

impl Solution {
    fn inner<'a>(s: &'a [u8], t: &'a [u8], n: NonZero<usize>, map: &mut impl Map<'a>) -> bool {
        for chunk in s.chunks_exact(n.get()) {
            map.inc(chunk);
        }

        for chunk in t.chunks_exact(n.get()) {
            if !map.dec(chunk) {
                return false;
            }
        }

        true
    }

    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = NonZero::new(s.len() / k.cast_unsigned() as usize).unwrap();

        match n.get() {
            1..4 => Self::inner(s, t, n, &mut [0_u32; 26 * 26 * 26]),
            4..7 => Self::inner(s, t, n, &mut HashMap::<u32, u32>::new()),
            7..14 => Self::inner(s, t, n, &mut HashMap::<u64, u32>::new()),
            14..28 => Self::inner(s, t, n, &mut HashMap::<u128, u32>::new()),
            _ => Self::inner(s, t, n, &mut HashMap::<&[u8], u32>::new()),
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        Self::is_possible_to_rearrange(s, t, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
