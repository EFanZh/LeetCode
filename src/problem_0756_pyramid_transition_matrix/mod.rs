pub mod block_by_block_backtracking;
pub mod cached_layer_by_layer_backtracking;
pub mod layer_by_layer_backtracking;

pub trait Solution {
    fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("BCD", &["BCC", "CDE", "CEA", "FFF"] as &[_]), true),
            (("AAAA", &["AAB", "AAC", "BCD", "BBE", "DEF"]), false),
            (
                (
                    "CBDDA",
                    &[
                        "ACC", "ACA", "AAB", "BCA", "BCB", "BAC", "BAA", "CAC", "BDA", "CAA", "CCA", "CCC", "CCB",
                        "DAD", "CCD", "DAB", "ACD", "DCA", "CAD", "CBB", "ABB", "ABC", "ABD", "BDB", "BBC", "BBA",
                        "DDA", "CDD", "CBC", "CBA", "CDA", "DBA", "ABA",
                    ],
                ),
                true,
            ),
            (("CDE", &["CDA", "DEA", "CDB", "BAA", "DEB"]), true),
            (
                (
                    "ABBBBA",
                    &[
                        "ACA", "ACF", "ACE", "ACD", "ABA", "ABF", "ABE", "ABD", "FCA", "FCF", "FCE", "FCD", "FBA",
                        "FBF", "FBE", "FBD", "ECA", "ECF", "ECE", "ECD", "EBA", "EBF", "EBE", "EBD", "DCA", "DCF",
                        "DCE", "DCD", "DBA", "DBF", "DBE", "DBD", "CAA", "CAF", "CAE", "CAD", "CFA", "CFF", "CFE",
                        "CFD", "CEA", "CEF", "CEE", "CED", "CDA", "CDF", "CDE", "CDD", "BAA", "BAF", "BAE", "BAD",
                        "BFA", "BFF", "BFE", "BFD", "BEA", "BEF", "BEE", "BED", "BDA", "BDF", "BDE", "BDD", "CCA",
                        "CCF", "CCE", "CCD", "CBA", "CBF", "CBE", "CBD", "BCA", "BCF", "BCE", "BCD", "BBA", "BBF",
                        "BBE", "BBD", "CCC", "CCB", "CBC", "CBB", "BCC", "BCB", "BBC", "BBB",
                    ],
                ),
                false,
            ),
            (("AAAA", &["AAB", "AAC", "BCD", "BBE", "DEF"]), false),
        ];

        for ((bottom, allowed), expected) in test_cases {
            assert_eq!(
                S::pyramid_transition(
                    bottom.to_string(),
                    allowed.iter().copied().map(str::to_string).collect()
                ),
                expected
            );
        }
    }
}
