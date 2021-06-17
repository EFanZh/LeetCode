pub mod doubly_linked_list;

pub trait AllOne {
    fn new() -> Self;
    fn inc(&mut self, key: String);
    fn dec(&mut self, key: String);
    fn get_max_key(&self) -> String;
    fn get_min_key(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::AllOne;

    enum Operation<'a> {
        Inc(&'a str),
        Dec(&'a str),
        GetMaxKey(&'a [&'a str]),
        GetMinKey(&'a [&'a str]),
    }

    #[allow(clippy::too_many_lines)]
    pub fn run<A: AllOne>() {
        use Operation::{Dec, GetMaxKey, GetMinKey, Inc};

        let test_cases = [
            &[
                Inc("hello"),
                Inc("hello"),
                GetMaxKey(&["hello"]),
                GetMinKey(&["hello"]),
                Inc("leet"),
                GetMaxKey(&["hello"]),
                GetMinKey(&["leet"]),
            ] as &[_],
            &[
                Inc("hello"),
                Inc("goodbye"),
                Inc("hello"),
                Inc("hello"),
                GetMaxKey(&["hello"]),
                Inc("leet"),
                Inc("code"),
                Inc("leet"),
                Dec("hello"),
                Inc("leet"),
                Inc("code"),
                Inc("code"),
                GetMaxKey(&["code", "hello", "leet"]),
            ],
            &[
                Inc("a"),
                Inc("b"),
                Inc("c"),
                Inc("d"),
                Inc("a"),
                Inc("b"),
                Inc("c"),
                Inc("d"),
                Inc("c"),
                Inc("d"),
                Inc("d"),
                Inc("a"),
                GetMinKey(&["b"]),
            ],
            &[
                Inc("hello"),
                Inc("world"),
                Inc("leet"),
                Inc("code"),
                Inc("DS"),
                Inc("leet"),
                GetMaxKey(&["leet"]),
                Inc("DS"),
                Dec("leet"),
                GetMaxKey(&["DS"]),
                Dec("DS"),
                Inc("hello"),
                GetMaxKey(&["hello"]),
                Inc("hello"),
                Inc("hello"),
                Dec("world"),
                Dec("leet"),
                Dec("code"),
                Dec("DS"),
                GetMaxKey(&["hello"]),
                Inc("new"),
                Inc("new"),
                Inc("new"),
                Inc("new"),
                Inc("new"),
                Inc("new"),
                GetMaxKey(&["new"]),
                GetMinKey(&["hello"]),
            ],
            &[
                Inc("a"),
                Inc("b"),
                Inc("b"),
                Inc("c"),
                Inc("c"),
                Inc("c"),
                Dec("b"),
                Dec("b"),
                GetMinKey(&["a"]),
                Dec("a"),
                GetMaxKey(&["c"]),
                GetMinKey(&["c"]),
            ],
            &[
                Inc("a"),
                Inc("b"),
                Inc("b"),
                Inc("b"),
                Inc("b"),
                Dec("b"),
                Dec("b"),
                GetMaxKey(&["b"]),
                GetMinKey(&["a"]),
            ],
            &[
                Inc("hello"),
                Inc("hello"),
                Inc("world"),
                Inc("world"),
                Inc("hello"),
                Dec("world"),
                GetMaxKey(&["hello"]),
                GetMinKey(&["world"]),
                Inc("world"),
                Inc("world"),
                Inc("leet"),
                GetMaxKey(&["hello", "world"]),
                GetMinKey(&["leet"]),
                Inc("leet"),
                Inc("leet"),
                GetMinKey(&["hello", "leet", "world"]),
            ],
            &[
                Inc("a"),
                Inc("b"),
                Inc("b"),
                Inc("c"),
                Inc("c"),
                Inc("c"),
                Inc("b"),
                GetMaxKey(&["b", "c"]),
                GetMinKey(&["a"]),
                Dec("b"),
                GetMaxKey(&["c"]),
                GetMinKey(&["a"]),
                Dec("a"),
                Dec("c"),
                Dec("b"),
                GetMaxKey(&["c"]),
                GetMinKey(&["b"]),
            ],
            &[
                Inc("a"),
                Dec("a"),
                GetMaxKey(&[""]),
                GetMinKey(&[""]),
                Dec("a"),
                GetMaxKey(&[""]),
                GetMinKey(&[""]),
            ],
        ];

        for operations in test_cases {
            let mut all_one = A::new();

            for operation in operations {
                match *operation {
                    Inc(key) => all_one.inc(key.to_string()),
                    Dec(key) => all_one.dec(key.to_string()),
                    GetMaxKey(keys) => assert!(keys.contains(&all_one.get_max_key().as_str())),
                    GetMinKey(keys) => assert!(keys.contains(&all_one.get_min_key().as_str())),
                }
            }
        }
    }
}
