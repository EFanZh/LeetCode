// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct TimeMap {
    inner: HashMap<String, (Vec<u32>, Vec<String>)>,
}

impl TimeMap {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let timestamp = timestamp as u32;

        match self.inner.entry(key) {
            Entry::Occupied(entry) => {
                let items = entry.into_mut();

                items.0.push(timestamp);
                items.1.push(value);
            }
            Entry::Vacant(entry) => {
                entry.insert((vec![timestamp], vec![value]));
            }
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let timestamp = timestamp as u32;

        self.inner
            .get(&key)
            .and_then(|(timestamps, values)| {
                values
                    .get(timestamps.partition_point(|&time| time <= timestamp).wrapping_sub(1))
                    .cloned()
            })
            .unwrap_or_default()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::TimeMap for TimeMap {
    fn new() -> Self {
        Self::new()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.set(key, value, timestamp);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.get(key, timestamp)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::TimeMap>();
    }
}
