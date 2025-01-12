pub mod topological_sorting_bfs;
pub mod topological_sorting_dfs;

pub trait Solution {
    fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::iter;

    type TestCase<'a> = ((i32, &'a [[i32; 2]], &'a [[i32; 2]]), bool);

    const EXTRA_TEST_CASE: TestCase = (
        (
            245,
            &[
                [107, 12],
                [180, 161],
                [236, 17],
                [226, 107],
                [11, 6],
                [23, 174],
                [160, 157],
                [162, 191],
                [60, 184],
                [174, 167],
                [241, 12],
                [150, 126],
                [47, 52],
                [39, 178],
                [142, 196],
                [121, 227],
                [106, 27],
                [213, 116],
                [245, 105],
                [208, 71],
                [59, 126],
                [137, 54],
                [198, 137],
                [35, 7],
                [245, 55],
                [35, 101],
                [159, 12],
                [18, 194],
                [192, 17],
                [222, 89],
                [86, 51],
                [82, 46],
                [173, 235],
                [30, 109],
                [42, 132],
                [14, 1],
                [105, 225],
                [179, 195],
                [204, 55],
                [213, 160],
                [144, 207],
                [220, 146],
                [41, 108],
                [191, 109],
                [134, 204],
                [237, 184],
                [130, 91],
                [50, 129],
                [237, 63],
                [175, 29],
                [5, 118],
                [104, 80],
                [142, 136],
                [210, 115],
                [76, 113],
                [28, 231],
                [149, 131],
                [37, 233],
                [97, 54],
                [48, 23],
                [12, 187],
                [149, 179],
                [177, 23],
                [44, 71],
                [226, 197],
                [205, 75],
                [145, 12],
                [102, 97],
                [76, 55],
                [60, 181],
                [172, 148],
                [172, 79],
                [233, 238],
                [24, 15],
                [165, 143],
                [9, 43],
                [76, 3],
                [180, 195],
                [125, 100],
                [230, 18],
                [8, 167],
                [35, 79],
                [42, 80],
                [183, 97],
                [5, 84],
                [134, 46],
                [145, 200],
                [69, 160],
                [65, 46],
                [180, 16],
                [60, 45],
                [14, 80],
                [245, 153],
                [65, 38],
                [194, 151],
                [159, 170],
                [53, 123],
                [46, 31],
                [28, 51],
                [150, 81],
                [109, 169],
                [162, 203],
                [118, 107],
                [120, 161],
                [37, 196],
                [239, 52],
                [215, 192],
                [32, 215],
                [220, 92],
                [56, 69],
                [69, 58],
                [35, 91],
                [206, 76],
                [5, 15],
                [189, 15],
                [14, 168],
                [22, 20],
                [218, 110],
                [176, 240],
                [153, 174],
                [198, 78],
                [179, 195],
                [180, 223],
                [114, 160],
                [107, 98],
                [77, 111],
                [120, 243],
                [188, 225],
                [136, 54],
                [201, 22],
                [180, 3],
                [226, 238],
                [118, 168],
                [236, 131],
                [77, 166],
                [125, 182],
                [120, 63],
                [5, 51],
                [173, 238],
                [211, 208],
                [227, 20],
                [82, 140],
                [219, 35],
                [214, 56],
                [9, 121],
                [198, 138],
                [35, 195],
                [130, 156],
                [93, 53],
                [216, 25],
                [215, 112],
                [56, 110],
                [85, 241],
                [120, 64],
                [227, 88],
                [91, 209],
                [128, 80],
                [161, 79],
                [4, 212],
                [141, 203],
                [168, 148],
                [180, 31],
                [227, 232],
                [93, 37],
                [71, 209],
                [125, 191],
                [28, 151],
                [45, 53],
                [89, 142],
                [189, 116],
                [203, 183],
                [34, 156],
                [10, 121],
                [162, 120],
                [95, 48],
                [24, 29],
                [162, 88],
                [188, 112],
                [134, 148],
                [168, 53],
                [168, 178],
                [48, 96],
                [136, 53],
                [82, 107],
                [211, 208],
                [245, 164],
                [226, 190],
                [240, 190],
                [138, 218],
                [197, 194],
                [133, 151],
                [143, 155],
                [2, 12],
                [23, 110],
                [223, 121],
                [191, 104],
                [182, 75],
                [233, 171],
                [169, 128],
                [88, 20],
                [2, 149],
                [18, 163],
                [117, 140],
                [132, 155],
                [156, 12],
                [117, 133],
                [203, 146],
                [11, 174],
                [241, 229],
                [125, 202],
                [208, 129],
                [13, 168],
                [162, 224],
                [120, 223],
                [13, 171],
                [176, 27],
                [154, 229],
                [89, 146],
                [100, 10],
                [222, 192],
                [11, 91],
                [180, 54],
                [220, 104],
                [120, 235],
                [30, 208],
                [154, 93],
                [211, 48],
                [9, 25],
                [185, 84],
                [44, 84],
                [5, 221],
                [194, 147],
                [121, 97],
                [159, 43],
                [124, 17],
                [150, 30],
                [176, 1],
                [146, 57],
                [3, 235],
                [129, 235],
                [238, 155],
                [176, 223],
                [197, 205],
                [223, 26],
                [125, 173],
                [42, 94],
                [53, 58],
                [85, 102],
                [196, 19],
                [21, 244],
                [122, 114],
                [87, 27],
                [32, 21],
                [143, 186],
                [28, 61],
                [41, 178],
                [51, 15],
                [228, 111],
                [133, 103],
                [45, 139],
                [5, 169],
                [23, 178],
                [189, 81],
                [210, 31],
                [240, 220],
                [226, 12],
                [143, 47],
                [197, 212],
                [191, 54],
                [168, 15],
                [67, 128],
                [135, 112],
                [146, 16],
                [117, 206],
                [145, 242],
                [61, 63],
                [39, 115],
                [63, 170],
                [73, 23],
                [24, 164],
                [65, 70],
                [32, 1],
                [64, 129],
                [146, 207],
                [130, 223],
                [235, 163],
                [206, 40],
                [181, 116],
                [185, 200],
                [100, 24],
                [165, 155],
                [218, 55],
                [122, 88],
                [219, 218],
                [28, 43],
                [215, 177],
                [135, 226],
                [183, 47],
                [7, 163],
                [39, 183],
                [82, 244],
                [13, 189],
                [109, 128],
                [85, 173],
                [149, 164],
                [40, 67],
                [171, 195],
                [39, 16],
                [67, 202],
                [165, 200],
                [221, 47],
                [115, 113],
                [34, 51],
                [132, 235],
                [66, 101],
                [104, 241],
                [189, 158],
                [222, 143],
                [187, 62],
                [78, 12],
                [74, 229],
                [176, 65],
                [42, 16],
                [234, 236],
                [113, 202],
                [130, 9],
                [118, 210],
                [208, 171],
                [23, 229],
                [138, 123],
                [93, 202],
                [150, 86],
                [125, 240],
                [185, 105],
                [33, 207],
                [94, 157],
                [47, 207],
                [86, 181],
                [39, 169],
                [103, 119],
                [123, 52],
                [217, 148],
                [228, 27],
                [99, 138],
                [139, 225],
                [236, 26],
                [41, 113],
                [10, 174],
                [74, 1],
                [194, 39],
                [81, 152],
                [122, 139],
                [223, 166],
                [78, 79],
                [41, 106],
                [117, 10],
                [233, 173],
                [81, 107],
                [219, 156],
                [107, 17],
                [191, 7],
                [87, 209],
                [135, 227],
                [13, 28],
                [14, 39],
                [242, 76],
                [18, 102],
                [33, 77],
                [87, 173],
                [35, 210],
                [161, 115],
                [66, 109],
                [144, 195],
                [245, 58],
                [32, 214],
                [149, 188],
                [222, 240],
                [151, 49],
                [87, 49],
                [234, 105],
                [5, 146],
                [128, 79],
                [188, 164],
                [74, 92],
                [72, 52],
                [95, 7],
                [162, 213],
                [82, 237],
                [49, 178],
                [106, 235],
                [28, 58],
                [182, 47],
                [226, 207],
                [7, 171],
                [217, 185],
                [2, 156],
                [50, 146],
                [70, 115],
                [3, 235],
                [201, 110],
                [11, 92],
                [40, 106],
                [152, 202],
                [61, 207],
                [125, 107],
                [50, 243],
                [40, 57],
                [161, 107],
                [231, 19],
                [154, 162],
                [72, 191],
                [215, 228],
                [127, 66],
                [120, 192],
                [60, 16],
                [16, 58],
                [41, 99],
                [85, 26],
                [151, 208],
                [66, 81],
                [3, 207],
                [104, 241],
                [180, 71],
                [107, 188],
                [194, 63],
                [5, 46],
                [49, 80],
                [208, 27],
                [60, 213],
                [181, 155],
                [191, 231],
                [219, 113],
                [229, 114],
                [182, 49],
                [177, 218],
                [169, 90],
                [18, 56],
                [242, 244],
                [18, 128],
                [205, 92],
                [82, 81],
                [159, 24],
                [230, 170],
                [239, 143],
                [73, 114],
                [38, 163],
                [219, 107],
                [14, 213],
                [142, 168],
                [129, 112],
                [109, 110],
                [118, 8],
                [193, 164],
                [63, 92],
                [42, 74],
                [111, 168],
                [214, 240],
                [117, 177],
                [205, 202],
                [228, 10],
                [103, 166],
                [159, 224],
                [211, 65],
                [189, 1],
                [138, 54],
                [238, 49],
                [36, 158],
                [6, 158],
                [242, 47],
                [204, 142],
                [13, 223],
                [201, 148],
                [91, 195],
                [116, 105],
                [135, 120],
                [41, 60],
                [42, 80],
                [149, 241],
                [81, 67],
                [182, 205],
                [65, 51],
                [176, 100],
                [143, 106],
                [133, 160],
                [176, 80],
                [77, 16],
                [11, 172],
                [129, 15],
                [150, 214],
                [13, 79],
                [183, 62],
                [134, 120],
                [204, 80],
                [242, 186],
                [199, 225],
                [189, 206],
                [193, 245],
                [153, 20],
                [37, 137],
                [117, 144],
                [242, 74],
                [61, 10],
                [24, 200],
                [107, 181],
                [39, 71],
                [82, 164],
                [228, 156],
                [38, 57],
                [183, 167],
                [20, 49],
                [7, 179],
                [125, 228],
                [151, 20],
                [238, 75],
                [6, 165],
                [141, 217],
                [44, 144],
                [166, 188],
                [18, 160],
                [216, 53],
                [236, 171],
                [149, 202],
                [127, 195],
                [127, 21],
                [77, 241],
                [130, 48],
                [22, 27],
                [204, 237],
                [61, 222],
                [194, 157],
                [13, 80],
                [21, 49],
                [150, 201],
                [193, 37],
                [234, 104],
                [223, 119],
                [66, 107],
                [76, 94],
                [151, 16],
                [74, 102],
                [245, 210],
                [229, 152],
                [102, 105],
                [182, 30],
                [67, 19],
                [68, 74],
                [135, 156],
                [30, 185],
                [94, 160],
            ],
            &[
                [59, 49],
                [149, 141],
                [66, 31],
                [169, 73],
                [39, 41],
                [189, 159],
                [66, 182],
                [245, 62],
                [43, 216],
                [57, 106],
                [80, 234],
                [103, 223],
                [76, 44],
                [158, 43],
                [47, 145],
                [27, 234],
                [237, 163],
                [15, 76],
                [134, 137],
                [149, 168],
                [3, 167],
                [239, 198],
                [159, 228],
                [21, 145],
                [26, 185],
                [13, 209],
                [176, 160],
                [188, 27],
                [244, 101],
                [136, 84],
                [27, 166],
                [187, 41],
                [24, 70],
                [205, 63],
                [47, 176],
                [94, 23],
                [96, 217],
                [200, 162],
                [161, 137],
                [190, 130],
                [98, 112],
                [20, 91],
                [109, 44],
                [197, 187],
                [16, 120],
                [195, 3],
                [98, 179],
                [242, 49],
                [22, 159],
                [203, 96],
                [242, 32],
                [221, 55],
                [88, 176],
                [215, 225],
                [42, 43],
                [202, 49],
                [94, 59],
                [245, 177],
                [78, 77],
                [90, 188],
                [102, 36],
                [141, 24],
                [28, 118],
                [59, 91],
                [122, 85],
                [107, 40],
                [224, 102],
                [39, 13],
                [103, 137],
                [180, 90],
                [58, 186],
                [245, 115],
                [234, 179],
                [110, 124],
                [208, 243],
                [2, 201],
                [105, 14],
                [214, 32],
                [205, 219],
                [190, 123],
                [191, 80],
                [81, 198],
                [115, 70],
                [61, 122],
                [3, 12],
                [1, 118],
                [142, 18],
                [22, 180],
                [67, 150],
                [187, 231],
                [227, 229],
                [113, 198],
                [52, 226],
                [140, 199],
                [73, 210],
                [29, 61],
                [58, 139],
                [33, 97],
                [133, 71],
                [185, 143],
                [211, 209],
                [218, 49],
                [6, 21],
                [192, 163],
                [195, 111],
                [147, 60],
                [132, 193],
                [111, 31],
                [230, 55],
                [169, 170],
                [52, 160],
                [236, 100],
                [10, 182],
                [81, 6],
                [135, 155],
                [88, 138],
                [237, 140],
                [28, 181],
                [35, 155],
                [200, 64],
                [19, 177],
                [213, 241],
                [180, 238],
                [105, 147],
                [102, 219],
                [30, 222],
                [158, 178],
                [28, 243],
                [88, 110],
                [167, 107],
                [68, 49],
                [177, 144],
                [70, 19],
                [165, 13],
                [75, 147],
                [102, 238],
                [156, 162],
                [94, 172],
                [226, 96],
                [138, 198],
                [160, 16],
                [20, 128],
                [158, 168],
                [52, 15],
                [205, 72],
                [25, 58],
                [82, 129],
                [149, 228],
                [156, 83],
                [83, 151],
                [77, 62],
                [232, 89],
                [123, 186],
                [25, 113],
                [54, 138],
                [170, 127],
                [84, 48],
                [105, 143],
                [111, 173],
                [54, 230],
                [2, 111],
                [142, 235],
                [230, 94],
                [20, 59],
                [191, 235],
                [7, 80],
                [66, 83],
                [84, 42],
                [166, 90],
                [3, 205],
                [205, 96],
                [155, 90],
                [157, 79],
                [61, 87],
                [53, 105],
                [146, 115],
                [72, 67],
                [220, 240],
                [48, 218],
                [232, 204],
                [147, 8],
                [210, 161],
                [31, 99],
                [68, 126],
                [38, 127],
                [8, 216],
                [151, 214],
                [127, 112],
                [135, 177],
                [45, 62],
                [43, 153],
                [232, 169],
                [7, 183],
                [29, 158],
                [222, 170],
                [16, 212],
                [162, 18],
                [80, 232],
                [78, 196],
                [106, 171],
                [12, 149],
                [197, 90],
                [10, 121],
                [157, 219],
                [233, 239],
                [232, 19],
                [47, 30],
                [165, 233],
                [31, 48],
                [15, 157],
                [103, 156],
                [65, 34],
                [180, 18],
                [118, 113],
                [230, 69],
                [68, 225],
                [85, 132],
                [123, 121],
                [139, 92],
                [221, 119],
                [10, 153],
                [18, 117],
                [166, 33],
                [51, 146],
                [151, 192],
                [92, 189],
                [52, 172],
                [50, 169],
                [214, 45],
                [144, 76],
                [60, 8],
                [170, 32],
                [77, 182],
                [202, 125],
                [114, 127],
                [2, 167],
                [191, 34],
                [27, 154],
                [114, 237],
                [132, 227],
                [42, 202],
                [84, 81],
                [91, 96],
                [37, 167],
                [149, 132],
                [74, 226],
                [180, 102],
                [189, 38],
                [185, 52],
                [235, 210],
                [45, 219],
                [96, 90],
                [172, 175],
                [104, 54],
                [91, 219],
                [47, 7],
                [26, 197],
                [214, 196],
                [176, 79],
                [175, 223],
                [28, 184],
                [68, 203],
                [138, 40],
                [98, 103],
                [204, 195],
                [118, 243],
                [189, 112],
                [41, 81],
                [76, 35],
                [108, 212],
                [27, 35],
                [69, 152],
                [201, 28],
                [125, 150],
                [111, 203],
                [196, 4],
                [48, 39],
                [61, 97],
                [11, 40],
                [130, 10],
                [83, 26],
                [185, 60],
                [118, 153],
                [158, 208],
                [103, 144],
                [200, 9],
                [170, 107],
                [46, 223],
                [160, 108],
                [17, 174],
                [167, 236],
                [160, 204],
                [57, 173],
                [93, 124],
                [156, 113],
                [78, 8],
                [168, 206],
                [183, 88],
                [113, 19],
                [185, 117],
                [98, 24],
                [169, 131],
                [185, 5],
                [227, 147],
                [200, 69],
                [180, 50],
                [66, 98],
                [105, 186],
                [105, 11],
                [159, 60],
                [42, 59],
                [26, 110],
                [15, 74],
                [46, 150],
                [206, 89],
                [136, 217],
                [146, 147],
                [172, 127],
                [147, 141],
                [202, 179],
                [163, 89],
                [188, 6],
                [241, 138],
                [112, 113],
                [91, 220],
                [175, 178],
                [204, 64],
                [202, 154],
                [161, 171],
                [125, 87],
                [103, 195],
                [244, 129],
                [5, 147],
                [84, 235],
                [45, 169],
                [102, 177],
                [12, 108],
                [215, 134],
                [75, 207],
                [5, 70],
                [14, 201],
                [38, 96],
                [88, 236],
                [88, 115],
                [222, 212],
                [27, 62],
                [225, 222],
                [231, 165],
                [173, 166],
                [86, 133],
                [92, 78],
                [45, 96],
                [80, 107],
                [27, 59],
                [172, 62],
                [204, 65],
                [186, 129],
                [157, 27],
                [122, 135],
                [67, 36],
                [87, 8],
                [218, 70],
                [115, 148],
                [109, 5],
                [103, 74],
                [188, 25],
                [37, 223],
                [83, 70],
                [82, 171],
                [2, 73],
                [98, 237],
                [84, 92],
                [67, 245],
                [245, 141],
                [159, 57],
                [156, 205],
                [132, 217],
                [218, 142],
                [244, 207],
                [231, 167],
                [235, 131],
                [41, 85],
                [179, 40],
                [53, 151],
                [146, 2],
                [222, 170],
                [84, 117],
                [139, 129],
                [130, 212],
                [205, 142],
                [94, 64],
                [3, 119],
                [118, 126],
                [136, 76],
                [124, 68],
                [61, 207],
                [182, 46],
                [51, 136],
                [54, 77],
                [173, 112],
                [31, 89],
                [143, 23],
                [80, 43],
                [13, 215],
                [111, 86],
                [11, 187],
                [101, 75],
                [188, 242],
                [233, 243],
                [93, 119],
                [130, 153],
                [66, 113],
                [177, 207],
                [233, 189],
                [80, 200],
                [10, 24],
                [191, 195],
                [184, 192],
                [188, 139],
                [209, 135],
                [139, 135],
                [70, 44],
                [228, 16],
                [87, 50],
                [39, 104],
                [214, 167],
                [34, 190],
                [141, 30],
                [228, 164],
                [156, 213],
                [45, 197],
                [13, 210],
                [200, 110],
                [79, 113],
                [98, 192],
                [153, 57],
                [180, 120],
                [31, 190],
                [167, 194],
                [182, 24],
                [110, 37],
                [196, 22],
                [45, 86],
                [200, 101],
                [215, 95],
                [197, 33],
                [10, 79],
                [234, 35],
                [143, 93],
                [236, 46],
                [203, 14],
                [172, 25],
                [206, 200],
                [1, 99],
                [211, 151],
                [113, 99],
                [182, 233],
                [180, 50],
                [102, 207],
                [5, 244],
                [98, 171],
                [70, 217],
                [211, 197],
                [82, 175],
                [168, 11],
                [52, 64],
                [106, 174],
                [210, 79],
                [235, 231],
                [26, 37],
                [36, 162],
                [223, 233],
                [2, 244],
                [129, 224],
                [105, 193],
                [88, 186],
                [165, 117],
                [179, 49],
                [138, 27],
                [208, 191],
                [105, 126],
                [206, 105],
                [208, 17],
                [64, 20],
                [234, 186],
                [210, 167],
                [43, 168],
                [63, 234],
                [107, 77],
                [18, 90],
                [53, 47],
                [4, 209],
                [51, 66],
                [154, 221],
                [19, 4],
                [242, 51],
                [80, 25],
                [59, 102],
                [142, 73],
                [99, 123],
                [83, 181],
                [51, 127],
                [81, 225],
                [40, 1],
                [30, 56],
                [109, 244],
                [61, 144],
                [240, 124],
                [7, 212],
                [79, 137],
                [196, 62],
                [185, 209],
                [131, 123],
                [204, 143],
                [204, 99],
                [123, 238],
                [128, 211],
                [234, 123],
                [202, 89],
                [111, 222],
                [227, 17],
                [226, 40],
                [18, 86],
                [56, 65],
                [89, 69],
                [141, 189],
                [72, 174],
                [180, 241],
                [101, 95],
                [157, 6],
                [71, 19],
                [137, 213],
                [23, 61],
                [139, 38],
                [206, 123],
                [86, 55],
                [31, 152],
                [124, 155],
                [81, 172],
                [83, 145],
                [114, 216],
                [100, 63],
                [156, 244],
                [217, 127],
                [142, 194],
                [132, 231],
                [151, 240],
                [208, 109],
                [185, 181],
                [63, 135],
                [199, 162],
                [26, 59],
                [227, 203],
                [96, 24],
                [160, 175],
                [58, 121],
                [21, 82],
                [59, 96],
                [141, 219],
                [159, 173],
                [172, 69],
                [64, 180],
                [74, 114],
                [229, 205],
                [20, 150],
                [179, 117],
                [66, 72],
                [134, 157],
                [47, 162],
                [13, 118],
                [131, 117],
                [78, 30],
                [28, 226],
                [99, 131],
                [26, 145],
                [26, 151],
                [216, 17],
                [211, 21],
                [6, 96],
                [196, 109],
                [66, 8],
                [15, 21],
                [27, 32],
                [11, 200],
                [166, 121],
                [140, 32],
                [115, 218],
                [30, 198],
                [5, 36],
                [20, 69],
                [105, 159],
                [29, 121],
                [9, 146],
                [185, 198],
                [130, 131],
                [132, 213],
                [73, 79],
                [84, 172],
                [28, 155],
                [179, 8],
                [134, 24],
                [178, 109],
                [17, 203],
                [152, 154],
                [200, 13],
                [136, 17],
                [15, 239],
                [240, 110],
                [156, 42],
                [143, 99],
                [160, 29],
                [241, 144],
                [242, 195],
                [97, 184],
                [160, 241],
                [149, 136],
                [195, 172],
                [179, 93],
                [161, 157],
                [109, 112],
                [199, 53],
                [92, 18],
                [59, 8],
                [15, 130],
                [48, 91],
                [180, 143],
                [221, 38],
                [156, 202],
                [134, 202],
                [227, 126],
                [167, 33],
                [43, 168],
            ],
        ),
        false,
    );

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[1, 2], [3, 2]] as &[_], &[[2, 1], [3, 2]] as &[_]), true),
            ((3, &[[1, 2], [2, 3], [3, 1], [2, 3]], &[[2, 1]]), false),
            EXTRA_TEST_CASE,
        ];

        let mut positions = Vec::new();

        for ((k, row_conditions, col_conditions), expected) in test_cases {
            let result = S::build_matrix(
                k,
                row_conditions.iter().map(Vec::from).collect(),
                col_conditions.iter().map(Vec::from).collect(),
            );

            if expected {
                positions.extend(iter::repeat((u16::MAX, u16::MAX)).take(k as u32 as usize));

                for (y, row) in result.into_iter().enumerate() {
                    for (x, num) in row.into_iter().enumerate() {
                        if num != 0 {
                            let position = &mut positions[num as u32 as usize - 1];

                            assert_eq!(*position, (u16::MAX, u16::MAX));

                            *position = (y as _, x as _);
                        }
                    }
                }

                assert!(positions.iter().all(|&position| position != (u16::MAX, u16::MAX)));

                for &[num_1, num_2] in row_conditions {
                    assert!(positions[num_1 as u32 as usize - 1].0 < positions[num_2 as u32 as usize - 1].0);
                }

                for &[num_1, num_2] in col_conditions {
                    assert!(positions[num_1 as u32 as usize - 1].1 < positions[num_2 as u32 as usize - 1].1);
                }

                positions.clear();
            } else {
                assert!(result.is_empty());
            }
        }
    }
}
