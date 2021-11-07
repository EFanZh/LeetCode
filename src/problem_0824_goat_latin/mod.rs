pub mod iterative;

pub trait Solution {
    fn to_goat_latin(sentence: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("I speak Goat Latin", "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"),
            ("The quick brown fox jumped over the lazy dog", "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"),
        ];

        for (sentence, expected) in test_cases {
            assert_eq!(S::to_goat_latin(sentence.to_string()), expected);
        }
    }
}
