// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct TimeMap {
    inner: HashMap<String, Vec<(u32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let event = (timestamp as _, value);

        match self.inner.entry(key) {
            Entry::Occupied(entry) => entry.into_mut().push(event),
            Entry::Vacant(entry) => {
                entry.insert(vec![event]);
            }
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let timestamp = timestamp as u32;

        self.inner
            .get(&key)
            .and_then(|events| {
                events
                    .get(events.partition_point(|&(time, _)| time <= timestamp).wrapping_sub(1))
                    .map(|(_, value)| value.clone())
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
