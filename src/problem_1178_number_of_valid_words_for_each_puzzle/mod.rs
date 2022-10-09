pub mod trie;

pub trait Solution {
    fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["aaaa", "asas", "able", "ability", "actt", "actor", "access"] as &[_],
                    &["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"] as &[_],
                ),
                &[1, 1, 3, 2, 4, 0] as &[_],
            ),
            (
                (
                    &["apple", "pleas", "please"],
                    &["aelwxyz", "aelpxyz", "aelpsxy", "saelpxy", "xaelpsy"],
                ),
                &[0, 1, 3, 2, 0],
            ),
            (
                (
                    &[
                        "aebd",
                        "cbdjegehgfcefbgceifdcjcbhdbbhhdi",
                        "bbcaehdgdghgaaghdbdg",
                        "fhgjegdagiadgdhaeicgjgifabadjdfe",
                        "gcadacg",
                        "efhjdffcaih",
                        "cgfjcdddabcdafjhcafieiadgebdeicbjjifgjbaf",
                        "cbfhbdaiajhdefgjefjibefjaahgdachhfge",
                        "cjejijcibgigceefidhcgbbdgg",
                        "jaedgdggbajbbibifadjeddbff",
                        "chdihgafjgfeaeefdigfeifjaihcg",
                        "giicgbjeah",
                        "bagcfechdabicgbidbceggjfedaabfibhcieefjhj",
                        "gbagbjhdjjdgifgaciehfjabi",
                        "ehheaajfbjdhabbjafeid",
                        "biajeeagdecjigefgidc",
                        "fejfdfeghbbdfc",
                        "bfbfffe",
                        "digd",
                        "bciabjhf",
                    ],
                    &[
                        "axtniqf", "xsdlyik", "ldsimca", "ptvdamy", "djihgak", "xencovd", "rbwpugz", "xvsbpmj",
                        "epfhmxs", "fshonmc",
                    ],
                ),
                &[0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            ),
        ];

        for ((words, puzzles), expected) in test_cases {
            assert_eq!(
                S::find_num_of_valid_words(
                    words.iter().copied().map(str::to_string).collect(),
                    puzzles.iter().copied().map(str::to_string).collect()
                ),
                expected,
            );
        }
    }
}
