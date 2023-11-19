pub mod bfs;

pub trait Solution {
    fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    #[allow(clippy::too_many_lines)] // Expected.
    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["java", "nodejs", "reactjs"] as &[_],
                    &[&["java"] as &[_], &["nodejs"], &["nodejs", "reactjs"]] as &[&[_]],
                ),
                2,
            ),
            (
                (
                    &["algorithms", "math", "java", "reactjs", "csharp", "aws"],
                    &[
                        &["algorithms", "math", "java"],
                        &["algorithms", "math", "reactjs"],
                        &["java", "csharp", "aws"],
                        &["reactjs", "csharp"],
                        &["csharp", "math"],
                        &["aws", "java"],
                    ],
                ),
                2,
            ),
            (
                (
                    &[
                        "cdkpfwkhlfbps",
                        "hnvepiymrmb",
                        "cqrdrqty",
                        "pxivftxovnpf",
                        "uefdllzzmvpaicyl",
                        "idsyvyl",
                    ],
                    &[
                        &[],
                        &["hnvepiymrmb"],
                        &["uefdllzzmvpaicyl"],
                        &[],
                        &["hnvepiymrmb", "cqrdrqty"],
                        &["pxivftxovnpf"],
                        &["hnvepiymrmb", "pxivftxovnpf"],
                        &["hnvepiymrmb"],
                        &["cdkpfwkhlfbps"],
                        &["idsyvyl"],
                        &[],
                        &["cdkpfwkhlfbps", "uefdllzzmvpaicyl"],
                        &["cdkpfwkhlfbps", "uefdllzzmvpaicyl"],
                        &["pxivftxovnpf", "uefdllzzmvpaicyl"],
                        &[],
                        &["cqrdrqty"],
                        &[],
                        &["cqrdrqty", "pxivftxovnpf", "idsyvyl"],
                        &["hnvepiymrmb", "idsyvyl"],
                        &[],
                    ],
                ),
                3,
            ),
            (
                (
                    &[
                        "hfkbcrslcdjq",
                        "jmhobexvmmlyyzk",
                        "fjubadocdwaygs",
                        "peaqbonzgl",
                        "brgjopmm",
                        "x",
                        "mf",
                        "pcfpppaxsxtpixd",
                        "ccwfthnjt",
                        "xtadkauiqwravo",
                        "zezdb",
                        "a",
                        "rahimgtlopffbwdg",
                        "ulqocaijhezwfr",
                        "zshbwqdhx",
                        "hyxnrujrqykzhizm",
                    ],
                    &[
                        &["peaqbonzgl", "xtadkauiqwravo"],
                        &["peaqbonzgl", "pcfpppaxsxtpixd", "zshbwqdhx"],
                        &["x", "a"],
                        &["a"],
                        &["jmhobexvmmlyyzk", "fjubadocdwaygs", "xtadkauiqwravo", "zshbwqdhx"],
                        &["fjubadocdwaygs", "x", "zshbwqdhx"],
                        &["x", "xtadkauiqwravo"],
                        &["x", "hyxnrujrqykzhizm"],
                        &["peaqbonzgl", "x", "pcfpppaxsxtpixd", "a"],
                        &["peaqbonzgl", "pcfpppaxsxtpixd"],
                        &["a"],
                        &["hyxnrujrqykzhizm"],
                        &["jmhobexvmmlyyzk"],
                        &["hfkbcrslcdjq", "xtadkauiqwravo", "a", "zshbwqdhx"],
                        &["peaqbonzgl", "mf", "a", "rahimgtlopffbwdg", "zshbwqdhx"],
                        &["xtadkauiqwravo"],
                        &["fjubadocdwaygs"],
                        &["x", "a", "ulqocaijhezwfr", "zshbwqdhx"],
                        &["peaqbonzgl"],
                        &["pcfpppaxsxtpixd", "ulqocaijhezwfr", "hyxnrujrqykzhizm"],
                        &["a", "ulqocaijhezwfr", "hyxnrujrqykzhizm"],
                        &["a", "rahimgtlopffbwdg"],
                        &["zshbwqdhx"],
                        &["fjubadocdwaygs", "peaqbonzgl", "brgjopmm", "x"],
                        &["hyxnrujrqykzhizm"],
                        &["jmhobexvmmlyyzk", "a", "ulqocaijhezwfr"],
                        &["peaqbonzgl", "x", "a", "ulqocaijhezwfr", "zshbwqdhx"],
                        &["mf", "pcfpppaxsxtpixd"],
                        &["fjubadocdwaygs", "ulqocaijhezwfr"],
                        &["fjubadocdwaygs", "x", "a"],
                        &["zezdb", "hyxnrujrqykzhizm"],
                        &["ccwfthnjt", "a"],
                        &["fjubadocdwaygs", "zezdb", "a"],
                        &[],
                        &["peaqbonzgl", "ccwfthnjt", "hyxnrujrqykzhizm"],
                        &["xtadkauiqwravo", "hyxnrujrqykzhizm"],
                        &["peaqbonzgl", "a"],
                        &["x", "a", "hyxnrujrqykzhizm"],
                        &["zshbwqdhx"],
                        &[],
                        &["fjubadocdwaygs", "mf", "pcfpppaxsxtpixd", "zshbwqdhx"],
                        &["pcfpppaxsxtpixd", "a", "zshbwqdhx"],
                        &["peaqbonzgl"],
                        &["peaqbonzgl", "x", "ulqocaijhezwfr"],
                        &["ulqocaijhezwfr"],
                        &["x"],
                        &["fjubadocdwaygs", "peaqbonzgl"],
                        &["fjubadocdwaygs", "xtadkauiqwravo"],
                        &["pcfpppaxsxtpixd", "zshbwqdhx"],
                        &["peaqbonzgl", "brgjopmm", "pcfpppaxsxtpixd", "a"],
                        &["fjubadocdwaygs", "x", "mf", "ulqocaijhezwfr"],
                        &["jmhobexvmmlyyzk", "brgjopmm", "rahimgtlopffbwdg", "hyxnrujrqykzhizm"],
                        &["x", "ccwfthnjt", "hyxnrujrqykzhizm"],
                        &["hyxnrujrqykzhizm"],
                        &[
                            "peaqbonzgl",
                            "x",
                            "xtadkauiqwravo",
                            "ulqocaijhezwfr",
                            "hyxnrujrqykzhizm",
                        ],
                        &["brgjopmm", "ulqocaijhezwfr", "zshbwqdhx"],
                        &["peaqbonzgl", "pcfpppaxsxtpixd"],
                        &["fjubadocdwaygs", "x", "a", "zshbwqdhx"],
                        &["fjubadocdwaygs", "peaqbonzgl", "x"],
                        &["ccwfthnjt"],
                    ],
                ),
                6,
            ),
        ];

        let mut all_skills = HashSet::<&str>::new();

        for ((req_skills, people), expected) in test_cases {
            let result = S::smallest_sufficient_team(
                req_skills.iter().copied().map(str::to_string).collect(),
                people
                    .iter()
                    .map(|skills| skills.iter().copied().map(str::to_string).collect())
                    .collect(),
            );

            assert_eq!(result.len(), expected);

            for i in result {
                all_skills.extend(people[i as usize]);
            }

            for &skill in req_skills {
                assert!(all_skills.contains(skill));
            }

            all_skills.clear();
        }
    }
}
