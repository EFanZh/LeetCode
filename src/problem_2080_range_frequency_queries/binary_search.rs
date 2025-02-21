// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct RangeFreqQuery {
    indices: HashMap<u16, Vec<u32>>,
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut indices = HashMap::<u16, Vec<u32>>::new();

        for (i, value) in (0..).zip(arr) {
            match indices.entry(value as _) {
                Entry::Occupied(entry) => entry.into_mut().push(i),
                Entry::Vacant(entry) => {
                    entry.insert(vec![i]);
                }
            }
        }

        Self { indices }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        self.indices.get(&(value as _)).map_or(0, |indices| {
            let start = indices.partition_point(|&x| x < left as u32);

            indices[start..].partition_point(|&x| x <= right as u32) as _
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::RangeFreqQuery for RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        Self::new(arr)
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        self.query(left, right, value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::RangeFreqQuery>();
    }
}
