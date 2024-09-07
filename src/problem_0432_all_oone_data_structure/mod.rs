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

    enum Operation {
        Inc(&'static str),
        Dec(&'static str),
        GetMaxKey(&'static [&'static str]),
        GetMinKey(&'static [&'static str]),
    }

    const EXTRA_TEST_CASE_1: &[Operation] = &[
        Operation::Inc("hello"),
        Operation::Inc("world"),
        Operation::Inc("leet"),
        Operation::Inc("code"),
        Operation::Inc("DS"),
        Operation::Inc("leet"),
        Operation::GetMaxKey(&["leet"]),
        Operation::Inc("DS"),
        Operation::Dec("leet"),
        Operation::GetMaxKey(&["DS"]),
        Operation::Dec("DS"),
        Operation::Inc("hello"),
        Operation::GetMaxKey(&["hello"]),
        Operation::Inc("hello"),
        Operation::Inc("hello"),
        Operation::Dec("world"),
        Operation::Dec("leet"),
        Operation::Dec("code"),
        Operation::Dec("DS"),
        Operation::GetMaxKey(&["hello"]),
        Operation::Inc("new"),
        Operation::Inc("new"),
        Operation::Inc("new"),
        Operation::Inc("new"),
        Operation::Inc("new"),
        Operation::Inc("new"),
        Operation::GetMaxKey(&["new"]),
        Operation::GetMinKey(&["hello"]),
    ];

    const EXTRA_TEST_CASE_2: &[Operation] = &[
        Operation::Inc("hello"),
        Operation::Inc("hello"),
        Operation::Inc("world"),
        Operation::Inc("world"),
        Operation::Inc("hello"),
        Operation::Dec("world"),
        Operation::GetMaxKey(&["hello"]),
        Operation::GetMinKey(&["world"]),
        Operation::Inc("world"),
        Operation::Inc("world"),
        Operation::Inc("leet"),
        Operation::GetMaxKey(&["hello", "world"]),
        Operation::GetMinKey(&["leet"]),
        Operation::Inc("leet"),
        Operation::Inc("leet"),
        Operation::GetMinKey(&["hello", "leet", "world"]),
    ];

    const EXTRA_TEST_CASE_3: &[Operation] = &[
        Operation::Inc("a"),
        Operation::Inc("b"),
        Operation::Inc("b"),
        Operation::Inc("c"),
        Operation::Inc("c"),
        Operation::Inc("c"),
        Operation::Inc("b"),
        Operation::GetMaxKey(&["b", "c"]),
        Operation::GetMinKey(&["a"]),
        Operation::Dec("b"),
        Operation::GetMaxKey(&["c"]),
        Operation::GetMinKey(&["a"]),
        Operation::Dec("a"),
        Operation::Dec("c"),
        Operation::Dec("b"),
        Operation::GetMaxKey(&["c"]),
        Operation::GetMinKey(&["b"]),
    ];

    pub fn run<A: AllOne>() {
        let test_cases = [
            &[
                Operation::Inc("hello"),
                Operation::Inc("hello"),
                Operation::GetMaxKey(&["hello"]),
                Operation::GetMinKey(&["hello"]),
                Operation::Inc("leet"),
                Operation::GetMaxKey(&["hello"]),
                Operation::GetMinKey(&["leet"]),
            ] as &[_],
            &[
                Operation::Inc("hello"),
                Operation::Inc("goodbye"),
                Operation::Inc("hello"),
                Operation::Inc("hello"),
                Operation::GetMaxKey(&["hello"]),
                Operation::Inc("leet"),
                Operation::Inc("code"),
                Operation::Inc("leet"),
                Operation::Dec("hello"),
                Operation::Inc("leet"),
                Operation::Inc("code"),
                Operation::Inc("code"),
                Operation::GetMaxKey(&["code", "hello", "leet"]),
            ],
            &[
                Operation::Inc("a"),
                Operation::Inc("b"),
                Operation::Inc("c"),
                Operation::Inc("d"),
                Operation::Inc("a"),
                Operation::Inc("b"),
                Operation::Inc("c"),
                Operation::Inc("d"),
                Operation::Inc("c"),
                Operation::Inc("d"),
                Operation::Inc("d"),
                Operation::Inc("a"),
                Operation::GetMinKey(&["b"]),
            ],
            &[
                Operation::Inc("a"),
                Operation::Inc("b"),
                Operation::Inc("b"),
                Operation::Inc("c"),
                Operation::Inc("c"),
                Operation::Inc("c"),
                Operation::Dec("b"),
                Operation::Dec("b"),
                Operation::GetMinKey(&["a"]),
                Operation::Dec("a"),
                Operation::GetMaxKey(&["c"]),
                Operation::GetMinKey(&["c"]),
            ],
            &[
                Operation::Inc("a"),
                Operation::Inc("b"),
                Operation::Inc("b"),
                Operation::Inc("b"),
                Operation::Inc("b"),
                Operation::Dec("b"),
                Operation::Dec("b"),
                Operation::GetMaxKey(&["b"]),
                Operation::GetMinKey(&["a"]),
            ],
            &[
                Operation::Inc("a"),
                Operation::Dec("a"),
                Operation::GetMaxKey(&[""]),
                Operation::GetMinKey(&[""]),
                Operation::Dec("a"),
                Operation::GetMaxKey(&[""]),
                Operation::GetMinKey(&[""]),
            ],
            EXTRA_TEST_CASE_1,
            EXTRA_TEST_CASE_2,
            EXTRA_TEST_CASE_3,
        ];

        for operations in test_cases {
            let mut all_one = A::new();

            for operation in operations {
                match *operation {
                    Operation::Inc(key) => all_one.inc(key.to_string()),
                    Operation::Dec(key) => all_one.dec(key.to_string()),
                    Operation::GetMaxKey(keys) => assert!(keys.contains(&all_one.get_max_key().as_str())),
                    Operation::GetMinKey(keys) => assert!(keys.contains(&all_one.get_min_key().as_str())),
                }
            }
        }
    }
}
