pub mod hash_map;
pub mod hash_map_2;

pub trait Solution {
    fn most_popular_creator(creators: Vec<String>, ids: Vec<String>, views: Vec<i32>) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    type TestCase<'a> = ((&'a [&'a str], &'a [&'a str], &'a [i32]), &'a [[&'a str; 2]]);

    const EXTRA_TEST_CASE: TestCase = (
        (
            &[
                "oa", "ve", "fnnt", "m", "o", "d", "wfb", "m", "b", "syvsm", "zrovn", "cdkoh", "cms", "rndi", "ru",
                "xv", "vds", "seh", "jazp", "i", "utpp", "d", "l", "jy", "xcm", "lfcis", "jkx", "d", "samt", "isaq",
                "eyzhr", "qtpy", "njrqd", "awu", "ihu", "p", "qsz", "x", "klb", "iyegx", "anx", "xt", "x", "o", "ge",
                "gyf", "yste", "ffu", "pfhra", "wp", "svo", "pc", "qschn", "kvtr", "lj", "xx", "tst", "wo", "ngvl",
                "jje", "evmi", "inzy", "vf", "nq", "iba", "ds", "i", "kdy", "jf", "esyev", "h", "wk", "xr", "w",
                "pgxyy", "pvu", "jzwc", "innd", "rcey", "z", "dk", "i", "ugad", "t", "mauj", "qsjre", "m", "slssv",
                "fzw", "pyex", "y", "tpu", "jrxn", "iik", "kjz", "af", "kyy", "uy", "xvw", "t", "e", "jklva", "vvr",
                "yb", "wxif", "rkxwb", "c", "sa", "lfi", "fyevh", "bkpak", "hdyh", "p", "xn", "gaj", "dmdpo", "wvnfc",
                "qbrnv", "spzus", "bnk", "qt", "teflb", "a", "q", "qisup", "tsuc", "cevza", "rb", "es", "gri", "w",
                "knru", "zo", "bt", "hskv", "b", "mu", "abcnn", "rag", "bjfm", "mzhsi", "wu", "tui", "guu", "jrkc",
                "uz", "qmil", "z", "vii", "trnr", "b", "irta", "pyk", "vvyh", "ykx", "h", "cdwr", "qsgrd", "pt", "dvp",
                "eylvj", "zqwu", "v", "knymw", "prh", "gz", "mqii", "mpks", "utcv", "obumh", "rkgz", "o", "wjzqw", "m",
                "v", "yz", "w", "rg", "vyiau", "kh", "wbjrz", "eyd", "qljqm", "jqjqs", "qwf", "rijb", "ik", "a", "y",
                "ddt", "hfbgw", "qlmla", "stpz", "ogozd", "xsa", "kp", "jemge", "xli", "rupvv", "ydxa", "ldu", "x",
                "f", "srhk", "hjrs", "pnh", "c", "h", "mjo", "w", "jffg", "wij", "jv", "hd", "it", "o", "kve", "b",
                "ivk", "bqsue", "tumri", "td", "lsrfd", "i", "mez", "fsjdm", "uwpfe", "bhq", "hmkkr", "szqhw", "curnw",
                "rnda", "d", "eu", "vvnfk", "mylu", "e", "zijk", "ps", "qtzrg", "yzsle", "fcuht", "zkf", "ey", "ygl",
                "tked", "rl", "qoa", "px", "set", "uzlhm", "navdj", "hrn", "zx", "b", "d", "rxl", "abzoq", "kgjmd",
                "qccno", "x", "uyqo", "zn", "glk", "djus", "yrjmr", "e", "gn", "nulqv", "ucre", "e", "jgu", "vtq",
                "jnpu", "ufu", "cz", "jolw", "hrhvr", "e", "nzmyp", "ji", "b", "bpk", "kkpq", "dfwgn", "jrx", "kmvyi",
                "bvbzl", "yoqw", "y", "y", "mzdkq", "nczh", "hbkv", "ce", "n", "wbghq", "pjkzi", "ew", "j", "wek",
                "jfrd", "gnv", "mjxtz", "lvjmt", "edz", "n", "uiv", "opv", "me", "tgmp", "ixvnx", "iqle", "bca",
                "yhipp", "e", "pz", "h", "ooq", "ffizu", "nq", "fm", "d", "ugtyy", "lzjn", "aeds", "k", "bmr", "jdvw",
                "mdp", "bfqw", "m", "yo", "dtfw", "xe", "wl", "ksuy", "ltw", "egbj", "gsia", "fejiy", "nd", "vb",
                "qltbc", "qj", "vei", "jtq", "wnsft", "zinn", "zgfum", "v",
            ],
            &[
                "p", "ic", "oh", "efsd", "a", "fhcw", "k", "qif", "wmh", "panih", "snmp", "aaxsj", "ysmsf", "pwa",
                "fagvg", "wrk", "dosj", "cuer", "f", "oby", "lfbur", "g", "dyq", "qejnf", "wken", "g", "ukcx", "f",
                "dyga", "lj", "tiajc", "sdcf", "nmsss", "v", "vhv", "eqjc", "lw", "rqzt", "xo", "b", "unvkr", "swqhv",
                "j", "lp", "mv", "wf", "uykzf", "dcj", "wu", "kwttt", "zca", "c", "tffd", "nlm", "m", "vptwh", "tzlbs",
                "owvhw", "flbn", "n", "q", "q", "fc", "riqt", "nfq", "lrrw", "smyl", "hzwis", "jl", "yx", "fcmh", "df",
                "jcg", "vskyb", "dirzd", "s", "rnp", "uykps", "bituj", "xxbzv", "pk", "a", "f", "axobp", "xb", "tskwm",
                "znyhz", "w", "m", "qsxe", "ry", "qeq", "xzuc", "s", "v", "zymmu", "ukeb", "nl", "zjguq", "hott",
                "cgcb", "kbzeo", "pzb", "laslx", "ljyt", "olek", "mjto", "xotfv", "zqjsn", "ldj", "n", "cqisb", "p",
                "h", "itp", "tap", "ny", "e", "rmf", "cmmh", "zxma", "y", "xfco", "cbgv", "lw", "au", "rehx", "yk",
                "hfqq", "gwsr", "i", "gfc", "wxqa", "sfzv", "qhj", "n", "ggqr", "uk", "oy", "pplju", "jza", "ktymm",
                "yc", "rygzq", "v", "doy", "h", "zjw", "uktua", "vm", "bzwey", "doen", "l", "onpj", "pl", "saq",
                "xenk", "sb", "whmvv", "yf", "my", "ohrnz", "uuwql", "h", "nerd", "krs", "vdqmx", "m", "t", "ycwgc",
                "cnxz", "s", "yjat", "ah", "tep", "m", "zgnu", "uvw", "bfm", "gsu", "tha", "jm", "v", "d", "hdl",
                "lhp", "ey", "we", "xodb", "zvhy", "nmy", "o", "gp", "hgpte", "gkctn", "yvvll", "xn", "n", "nbfc",
                "uxidu", "aubn", "hwlly", "q", "vqu", "v", "knp", "pws", "rgi", "hf", "go", "sf", "r", "jg", "rv",
                "rteaw", "l", "aev", "zd", "dix", "f", "qrf", "zj", "iszax", "hi", "iykk", "ffgsd", "gad", "qd", "cg",
                "xi", "txp", "wfxru", "kxs", "pfm", "ftssj", "mm", "wxl", "s", "vas", "im", "so", "v", "cns", "vc",
                "xvpe", "yjqp", "tmclc", "rfwvr", "apwae", "sp", "kuj", "qhp", "lisa", "v", "s", "pvz", "dtlk", "t",
                "fd", "v", "ypa", "dwkgx", "mrasy", "h", "bra", "jkes", "locwf", "mp", "wa", "hriic", "ezs", "vvbyp",
                "pjf", "qgl", "isu", "wouca", "gyv", "hkpkn", "try", "f", "y", "nrsj", "nh", "lk", "xdh", "hl", "n",
                "wpfee", "khhj", "lfhu", "ujaa", "jludd", "id", "aqpg", "d", "bgin", "qiohm", "dhft", "ym", "zuob",
                "w", "apgy", "ey", "c", "huf", "h", "j", "e", "g", "g", "zfp", "fm", "l", "iulvj", "pm", "kxm",
                "lqiro", "ckwuo", "cqiog", "atb", "x", "q", "zi", "abr", "wkm", "wj", "gw", "mqiy", "gs", "mpa", "vn",
                "jj", "kodrx", "yg", "zl", "pov", "otf", "ulv", "yvxit", "o", "xz", "ifmz", "jriv", "etfw", "l", "f",
                "qq", "ak", "lvq", "zjl", "lg",
            ],
            &[
                758, 561, 416, 46, 463, 784, 447, 999, 169, 269, 390, 418, 79, 71, 89, 310, 374, 65, 35, 222, 207, 799,
                655, 889, 269, 681, 492, 853, 325, 261, 537, 415, 190, 576, 303, 12, 264, 376, 863, 595, 960, 696, 260,
                305, 539, 276, 791, 859, 88, 206, 254, 634, 496, 162, 333, 243, 238, 534, 861, 5, 58, 728, 816, 397,
                130, 301, 12, 524, 765, 905, 477, 688, 398, 794, 388, 674, 387, 946, 871, 753, 68, 265, 336, 574, 191,
                102, 951, 990, 973, 994, 265, 101, 247, 129, 269, 136, 694, 81, 345, 787, 787, 677, 110, 791, 313, 807,
                179, 50, 566, 864, 728, 413, 363, 39, 804, 611, 196, 599, 981, 183, 907, 235, 268, 118, 501, 679, 572,
                544, 174, 324, 958, 874, 567, 74, 675, 588, 534, 968, 814, 727, 638, 533, 820, 675, 907, 254, 37, 526,
                473, 441, 815, 243, 95, 20, 417, 149, 41, 28, 129, 349, 643, 498, 822, 612, 502, 532, 750, 21, 988,
                966, 349, 945, 456, 395, 308, 196, 994, 94, 779, 275, 961, 814, 653, 433, 692, 313, 661, 395, 126, 836,
                532, 835, 356, 198, 928, 998, 416, 507, 799, 973, 608, 29, 269, 569, 910, 217, 69, 893, 524, 834, 1000,
                646, 835, 937, 546, 181, 603, 858, 495, 182, 642, 356, 897, 942, 18, 1, 56, 512, 920, 688, 961, 672,
                198, 648, 998, 912, 708, 928, 327, 351, 834, 624, 242, 64, 424, 998, 169, 814, 908, 278, 80, 47, 404,
                967, 648, 660, 710, 558, 376, 707, 957, 4, 868, 10, 516, 752, 248, 137, 42, 177, 47, 545, 252, 262,
                953, 641, 591, 283, 935, 957, 196, 483, 529, 621, 272, 620, 544, 845, 191, 363, 404, 435, 624, 32, 774,
                327, 561, 362, 147, 244, 945, 818, 990, 844, 782, 486, 881, 187, 544, 674, 501, 244, 865, 766, 732,
                117, 108, 497, 956, 946, 86, 724, 530, 433, 352, 36, 174, 272, 7, 109, 536, 915, 523, 342, 962, 222,
                485, 292, 260, 415, 728, 376, 668, 849, 129, 489, 777, 290, 17, 367, 822,
            ],
        ),
        &[["d", "f"]],
    );

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["alice", "bob", "alice", "chris"] as &[_],
                    &["one", "two", "three", "four"] as &[_],
                    &[5, 10, 5, 4] as &[_],
                ),
                &[["alice", "one"], ["bob", "two"]] as &[_],
            ),
            (
                (&["alice", "alice", "alice"], &["a", "b", "c"], &[1, 2, 2]),
                &[["alice", "b"]],
            ),
            EXTRA_TEST_CASE,
        ];

        for ((creators, ids, views), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::most_popular_creator(
                    creators.iter().copied().map(str::to_string).collect(),
                    ids.iter().copied().map(str::to_string).collect(),
                    views.to_vec(),
                )),
                expected,
            );
        }
    }
}
