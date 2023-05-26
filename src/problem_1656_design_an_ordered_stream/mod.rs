pub mod vec_deque;

pub trait OrderedStream {
    fn new(n: i32) -> Self;
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::OrderedStream;

    pub fn run<S: OrderedStream>() {
        let test_cases = [
            (
                5,
                &[
                    ((3, "ccccc"), &[] as &[_]),
                    ((1, "aaaaa"), &["aaaaa"]),
                    ((2, "bbbbb"), &["bbbbb", "ccccc"]),
                    ((5, "eeeee"), &[]),
                    ((4, "ddddd"), &["ddddd", "eeeee"]),
                ] as &[(_, &[_])],
            ),
            (
                9,
                &[
                    ((7, "hhfrg"), &[]),
                    ((4, "atnke"), &[]),
                    ((3, "xeizc"), &[]),
                    ((2, "mgftc"), &[]),
                    ((5, "vrqpq"), &[]),
                    ((9, "mlbxv"), &[]),
                    ((1, "zghqy"), &["zghqy", "mgftc", "xeizc", "atnke", "vrqpq"]),
                    ((8, "uavkc"), &[]),
                    ((6, "xlljl"), &["xlljl", "hhfrg", "uavkc", "mlbxv"]),
                ],
            ),
        ];

        for (n, inserts) in test_cases {
            let mut ordered_stream = S::new(n);

            for &((id_key, value), expected) in inserts {
                assert_eq!(ordered_stream.insert(id_key, value.to_string()), expected);
            }
        }
    }
}
