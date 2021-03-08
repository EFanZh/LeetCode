pub mod hash_maps;
pub mod hash_maps_online_hack;

pub trait Codec {
    fn new() -> Self;
    fn encode(&mut self, long_url: String) -> String;
    fn decode(&self, long_url: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Codec;

    pub fn run<C: Codec>() {
        let test_cases = [
            &[
                "https://leetcode.com/problems/design-tinyurl",
                "https://leetcode.com/problems/encode-and-decode-tinyurl/",
            ] as &[_],
            &["http://tinyurl.com/4e9iAk", "http://tinyurl.com/4e9iAk"],
            &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
                "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d",
                "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x",
                "y", "z",
            ],
        ];

        for urls in test_cases.iter().copied() {
            let mut codec = C::new();

            for &url in urls {
                let encoded = codec.encode(url.to_string());

                assert_eq!(codec.decode(encoded), url);
            }
        }
    }
}
