pub mod dfs;
pub mod dfs_2;
pub mod dfs_3;

pub trait Solution {
    fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[["MUC", "LHR"], ["JFK", "MUC"], ["SFO", "SJC"], ["LHR", "SFO"]] as &[_],
                &["JFK", "MUC", "LHR", "SFO", "SJC"] as &[_],
            ),
            (
                &[
                    ["JFK", "SFO"],
                    ["JFK", "ATL"],
                    ["SFO", "ATL"],
                    ["ATL", "JFK"],
                    ["ATL", "SFO"],
                ],
                &["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"],
            ),
            (
                &[["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]],
                &["JFK", "NRT", "JFK", "KUL"],
            ),
        ];

        for (tickets, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::find_itinerary(
                    tickets
                        .iter()
                        .map(|&[from, to]| vec![from.to_string(), to.to_string()])
                        .collect()
                ),
                expected
            );
        }
    }
}
