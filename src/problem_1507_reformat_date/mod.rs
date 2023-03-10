pub mod parsing;

pub trait Solution {
    fn reformat_date(date: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("20th Oct 2052", "2052-10-20"),
            ("6th Jun 1933", "1933-06-06"),
            ("26th May 1960", "1960-05-26"),
            ("22nd Apr 2023", "2023-04-22"),
            ("4th Aug 2048", "2048-08-04"),
            ("16th Dec 2018", "2018-12-16"),
            ("1st Feb 1997", "1997-02-01"),
            ("10th Jan 2082", "2082-01-10"),
            ("28th Jul 1963", "1963-07-28"),
            ("16th Mar 2068", "2068-03-16"),
            ("4th Nov 2030", "2030-11-04"),
            ("20th Sep 1958", "1958-09-20"),
        ];

        for (date, expected) in test_cases {
            assert_eq!(S::reformat_date(date.to_string()), expected);
        }
    }
}
