pub mod binary_search;
pub mod binary_search_2;

pub trait TimeMap {
    fn new() -> Self;
    fn set(&mut self, key: String, value: String, timestamp: i32);
    fn get(&self, key: String, timestamp: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::TimeMap;

    enum Operation<'a> {
        Set(&'a str, &'a str, i32),
        Get(&'a str, i32, &'a str),
    }

    pub fn run<M: TimeMap>() {
        let test_cases = [(&[
            Operation::Set("foo", "bar", 1),
            Operation::Get("foo", 1, "bar"),
            Operation::Get("foo", 3, "bar"),
            Operation::Set("foo", "bar2", 4),
            Operation::Get("foo", 4, "bar2"),
            Operation::Get("foo", 5, "bar2"),
        ] as &[_])];

        for operations in test_cases {
            let mut time_map = M::new();

            for operation in operations {
                match *operation {
                    Operation::Set(key, value, timestamp) => {
                        time_map.set(key.to_string(), value.to_string(), timestamp);
                    }
                    Operation::Get(key, timestamp, expected) => {
                        assert_eq!(time_map.get(key.to_string(), timestamp), expected);
                    }
                }
            }
        }
    }
}
