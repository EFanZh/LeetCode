pub mod dynamic_programming_and_sliding_window;

pub trait Solution {
    fn box_delivering(boxes: Vec<Vec<i32>>, ports_count: i32, max_boxes: i32, max_weight: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    type TestCase = ((&'static [[i32; 2]], i32, i32, i32), i32);

    const LONG_TEST_CASE_1: TestCase = (
        (
            &[
                [61, 4840],
                [66, 16490],
                [54, 15479],
                [48, 5555],
                [25, 3120],
                [47, 1010],
                [30, 3236],
                [7, 9270],
                [55, 14900],
                [48, 11603],
                [104, 12299],
                [3, 8266],
                [3, 1440],
                [15, 5659],
                [72, 13285],
                [32, 10642],
                [26, 5780],
                [88, 15220],
                [16, 3808],
                [27, 11203],
                [41, 7645],
                [79, 10232],
                [73, 403],
                [86, 11181],
                [25, 5789],
                [12, 15415],
                [59, 2075],
                [60, 11185],
                [39, 2213],
                [103, 12049],
                [99, 9585],
                [40, 16489],
                [71, 3282],
                [47, 10552],
                [10, 11910],
                [86, 7606],
                [6, 654],
                [30, 14945],
                [14, 3796],
                [22, 5430],
                [10, 13458],
                [74, 2169],
                [81, 10010],
                [92, 6330],
                [104, 778],
                [100, 3311],
                [98, 5975],
                [56, 15520],
                [13, 11700],
                [19, 6890],
                [99, 2910],
                [62, 1393],
                [48, 5638],
                [9, 10967],
                [38, 945],
                [23, 14549],
                [43, 4081],
                [42, 4540],
                [82, 5832],
                [69, 5072],
                [19, 15047],
                [85, 4330],
                [57, 3549],
                [32, 6955],
                [46, 15456],
                [80, 1358],
                [58, 25],
                [95, 9401],
                [10, 15268],
                [32, 12504],
                [10, 4724],
                [83, 4816],
                [45, 2084],
                [33, 7725],
                [32, 3637],
                [103, 7506],
                [103, 51],
                [69, 6945],
                [42, 4017],
                [66, 5596],
                [83, 305],
                [56, 10441],
                [70, 3892],
                [78, 2290],
                [50, 6269],
                [23, 14932],
                [17, 11895],
                [43, 3679],
                [39, 3086],
                [43, 16224],
                [12, 13993],
                [92, 14876],
                [6, 1219],
                [65, 8544],
                [25, 658],
                [79, 5722],
                [19, 14103],
                [80, 16496],
                [56, 8778],
                [44, 4481],
                [36, 14814],
                [77, 2370],
                [2, 7206],
                [100, 1700],
                [24, 1636],
                [36, 8805],
                [46, 7068],
                [7, 13167],
                [45, 8375],
                [63, 9633],
                [83, 8546],
                [13, 15183],
                [73, 14140],
                [12, 1395],
                [101, 2581],
                [2, 5718],
                [16, 2783],
                [34, 9200],
                [42, 10048],
                [74, 1353],
                [74, 2485],
                [33, 4091],
                [21, 9159],
                [79, 10195],
                [1, 9576],
                [63, 11647],
                [104, 5794],
                [103, 2786],
                [46, 121],
                [23, 5173],
                [35, 7066],
                [13, 12041],
                [51, 9573],
                [56, 2992],
                [81, 4133],
                [58, 9161],
                [27, 9496],
                [86, 4972],
                [33, 11241],
                [88, 11329],
                [69, 3844],
                [47, 1487],
                [94, 15931],
                [48, 11569],
                [15, 2003],
                [26, 11104],
                [33, 6961],
                [6, 15453],
                [2, 11193],
                [14, 3942],
                [94, 10791],
                [71, 1871],
                [98, 13280],
                [73, 8641],
                [21, 9413],
                [22, 12239],
                [38, 14552],
                [92, 14876],
                [43, 4579],
                [21, 12583],
                [67, 13959],
                [71, 9938],
            ],
            106,
            74,
            16517,
        ),
        261,
    );

    const LONG_TEST_CASE_2: TestCase = (
        (
            &[
                [41, 32035],
                [29, 34006],
                [15, 42248],
                [53, 13630],
                [3, 20604],
                [32, 33913],
                [23, 13746],
                [18, 3381],
                [45, 36702],
                [8, 21918],
                [25, 36179],
                [47, 14627],
                [62, 20196],
                [33, 39697],
                [17, 34359],
                [57, 21025],
                [64, 41254],
                [13, 22044],
                [25, 28742],
                [41, 41954],
                [60, 35831],
                [12, 26238],
                [25, 23371],
                [11, 21530],
                [13, 19199],
                [41, 17932],
                [26, 157],
                [64, 29116],
                [21, 12712],
                [63, 9896],
                [16, 34518],
                [16, 27712],
                [20, 12378],
                [61, 26493],
                [50, 4096],
                [48, 9011],
                [40, 23808],
                [19, 23150],
                [48, 25008],
                [33, 29110],
                [53, 5249],
                [63, 39008],
                [22, 35072],
                [5, 31169],
                [51, 36734],
                [55, 11071],
                [15, 40951],
                [41, 20257],
                [31, 28269],
                [14, 3082],
                [13, 10463],
                [41, 3155],
                [5, 30180],
                [31, 31882],
                [34, 30540],
                [61, 39097],
                [11, 10032],
                [21, 24292],
                [43, 8274],
                [5, 28860],
                [56, 20796],
                [21, 20034],
                [33, 1237],
                [52, 20025],
                [46, 16709],
                [25, 19108],
                [8, 40137],
                [40, 14829],
                [22, 7822],
                [21, 6929],
                [19, 34635],
                [46, 15905],
                [54, 28074],
                [23, 26664],
                [19, 28443],
                [14, 39560],
                [28, 12822],
                [5, 9500],
                [48, 8373],
                [55, 24406],
                [40, 9545],
                [62, 27397],
                [22, 34104],
                [61, 3624],
                [46, 8625],
                [37, 20146],
                [55, 36014],
                [55, 1938],
                [11, 24402],
                [30, 33373],
                [41, 16064],
                [17, 15289],
                [56, 40892],
                [45, 29712],
                [5, 21702],
                [20, 3514],
                [10, 18538],
                [24, 40944],
                [52, 28731],
                [39, 25767],
                [36, 33060],
                [30, 41143],
                [64, 26036],
                [21, 13367],
                [50, 11461],
                [31, 19809],
                [23, 5297],
                [59, 4961],
                [2, 32102],
                [63, 2371],
                [19, 8303],
                [30, 7868],
                [47, 31854],
                [29, 31075],
                [49, 3526],
                [64, 36208],
                [36, 16094],
                [15, 41030],
                [5, 22859],
                [3, 40429],
                [28, 40921],
                [54, 28316],
                [10, 42363],
                [60, 17030],
                [34, 22504],
                [41, 9861],
                [37, 8482],
                [25, 28443],
                [57, 29191],
                [22, 18029],
                [7, 9205],
                [60, 11252],
                [25, 4977],
                [26, 15914],
                [12, 31683],
                [46, 6366],
                [7, 20352],
                [6, 29863],
                [34, 42117],
                [26, 33120],
                [17, 4093],
                [23, 22773],
                [48, 24952],
                [51, 15682],
                [4, 32309],
                [59, 33376],
                [18, 4569],
                [46, 39629],
                [46, 27230],
                [32, 5575],
                [6, 22851],
                [48, 16756],
                [30, 19709],
                [59, 22383],
                [61, 3341],
                [15, 12537],
                [55, 15353],
                [37, 14092],
                [62, 14941],
                [24, 41854],
                [52, 28405],
                [21, 42453],
                [54, 26428],
                [41, 42233],
                [43, 19581],
                [36, 9092],
                [58, 23918],
                [5, 14502],
                [43, 5019],
                [57, 33060],
                [9, 10455],
                [31, 2739],
                [48, 28798],
                [44, 35986],
                [51, 8349],
                [9, 12200],
                [59, 32558],
                [21, 31706],
                [6, 25461],
                [46, 31870],
                [2, 9295],
                [9, 31041],
                [22, 4077],
                [37, 33710],
                [33, 1692],
                [53, 3108],
                [17, 14890],
                [15, 8117],
                [42, 24098],
                [53, 11705],
                [60, 16063],
                [23, 37971],
                [17, 22867],
                [52, 37307],
                [8, 6159],
                [36, 22964],
                [21, 29201],
                [1, 31773],
                [16, 18925],
                [53, 38446],
                [13, 34408],
                [41, 21044],
                [60, 5070],
                [44, 2338],
                [37, 10063],
                [44, 34120],
                [64, 6219],
                [55, 35739],
                [14, 21291],
                [49, 30379],
                [46, 24644],
                [55, 5697],
                [13, 40779],
                [7, 4414],
                [31, 14276],
                [48, 37542],
                [63, 30721],
                [22, 29283],
                [55, 1029],
                [33, 26423],
                [54, 7599],
                [21, 20907],
                [10, 27573],
                [20, 32844],
                [28, 4381],
                [34, 41430],
                [14, 9185],
                [21, 21529],
                [7, 3059],
                [53, 35956],
                [26, 39193],
                [3, 2751],
                [42, 27228],
                [55, 13700],
                [30, 39442],
                [16, 22931],
                [5, 8151],
                [38, 33462],
                [16, 12890],
                [23, 9595],
                [1, 7117],
                [23, 24916],
                [44, 16566],
                [33, 28672],
                [60, 39522],
                [13, 4275],
                [44, 12623],
                [42, 23346],
                [31, 32080],
                [34, 3772],
                [13, 14584],
                [10, 29236],
                [55, 13517],
                [15, 11427],
                [27, 11894],
                [17, 22558],
                [21, 14983],
                [42, 15553],
                [2, 32324],
            ],
            64,
            125,
            42502,
        ),
        431,
    );

    const LONG_TEST_CASE_3: TestCase = (
        (
            &[
                [179, 7243],
                [205, 17888],
                [138, 13004],
                [88, 10199],
                [103, 39432],
                [27, 27228],
                [100, 10336],
                [154, 16730],
                [213, 36824],
                [56, 14157],
                [139, 44285],
                [106, 33434],
                [122, 433],
                [69, 32992],
                [171, 13690],
                [54, 33237],
                [6, 32159],
                [172, 9907],
                [104, 1143],
                [16, 4476],
                [134, 13953],
                [28, 13027],
                [9, 1727],
                [216, 6847],
                [113, 6177],
                [99, 22582],
                [82, 26863],
                [132, 16460],
                [180, 28236],
                [132, 31569],
                [188, 19328],
                [100, 31674],
                [108, 24331],
                [166, 15254],
                [34, 42089],
                [118, 40971],
                [73, 39603],
                [57, 20045],
                [146, 45397],
                [46, 45797],
                [18, 37124],
                [87, 751],
                [8, 11372],
                [77, 31694],
                [9, 3674],
                [17, 4783],
                [44, 46608],
                [205, 12982],
                [178, 14817],
                [148, 2763],
                [97, 43969],
                [158, 40869],
                [127, 37418],
                [74, 21179],
                [194, 31732],
                [90, 33169],
                [112, 42632],
                [76, 44347],
                [124, 22602],
                [177, 25118],
                [112, 37623],
                [216, 12001],
                [14, 4818],
                [124, 26518],
                [68, 41429],
                [36, 31701],
                [157, 45400],
                [23, 20090],
                [40, 5189],
                [205, 4203],
                [150, 29106],
                [185, 16388],
                [53, 3540],
                [183, 2875],
                [52, 45876],
                [156, 32816],
                [112, 30184],
                [39, 22735],
                [85, 21660],
                [158, 46620],
                [137, 14775],
                [20, 8403],
                [189, 40078],
                [25, 3640],
                [105, 36757],
                [80, 16847],
                [170, 3660],
                [216, 9262],
                [191, 16838],
                [169, 10741],
                [5, 39390],
                [158, 10903],
                [16, 7077],
                [213, 466],
                [11, 29809],
                [9, 20859],
                [59, 2775],
                [36, 19932],
                [161, 46199],
                [175, 24625],
                [76, 40231],
                [178, 35648],
                [194, 35105],
                [40, 44728],
                [3, 41636],
                [121, 27469],
                [16, 9395],
                [114, 44182],
                [195, 11158],
                [169, 31139],
                [119, 20037],
                [130, 24959],
                [112, 12236],
                [91, 18326],
                [7, 16690],
                [213, 12294],
                [178, 47091],
                [119, 28579],
                [152, 22393],
                [94, 18992],
                [132, 43471],
                [5, 26759],
                [186, 16988],
                [123, 45040],
                [45, 18628],
                [94, 38813],
                [9, 23755],
                [152, 34555],
                [81, 43925],
                [215, 13423],
                [141, 30617],
                [47, 33711],
                [206, 8713],
                [210, 36402],
                [170, 26984],
                [119, 11984],
                [10, 25019],
                [164, 18286],
                [60, 44768],
                [149, 25669],
                [55, 38262],
                [38, 5791],
                [214, 46482],
                [40, 22284],
                [115, 41833],
                [201, 43977],
                [91, 17717],
                [206, 19353],
                [186, 11866],
                [177, 11296],
                [63, 17638],
                [34, 27963],
                [76, 42343],
                [126, 18664],
                [86, 3655],
                [75, 4148],
                [195, 44050],
                [166, 28298],
                [161, 40747],
                [215, 40475],
                [193, 35323],
                [32, 22873],
                [10, 39749],
                [107, 13640],
                [108, 42742],
                [44, 22870],
                [46, 38176],
                [213, 34137],
                [157, 37407],
                [24, 16568],
                [118, 22507],
                [46, 9414],
                [60, 190],
                [180, 15616],
                [33, 36649],
                [206, 43212],
                [24, 45404],
                [149, 22485],
                [17, 10092],
                [99, 22466],
                [6, 7394],
                [184, 22507],
                [80, 7992],
                [6, 32349],
                [78, 3612],
                [170, 15659],
                [196, 9173],
                [7, 45835],
                [6, 21727],
                [85, 11160],
                [189, 39970],
                [70, 15644],
                [102, 3882],
                [141, 15349],
                [147, 44934],
                [42, 25726],
                [58, 31620],
                [151, 25060],
                [168, 28125],
                [118, 29680],
                [30, 15172],
                [189, 33730],
                [109, 20561],
                [153, 37771],
                [144, 26350],
                [70, 4349],
                [56, 42950],
                [168, 19671],
                [26, 14734],
                [148, 23635],
                [143, 29237],
                [101, 32712],
                [164, 39808],
                [17, 10810],
                [62, 41754],
                [211, 30292],
                [176, 37845],
                [3, 23610],
                [173, 6492],
                [71, 17504],
                [32, 4308],
                [84, 36087],
                [113, 22114],
                [85, 31422],
                [136, 38144],
                [50, 9973],
                [52, 1922],
                [143, 21172],
                [145, 19700],
                [28, 35874],
                [86, 2557],
                [50, 18589],
                [208, 34543],
                [128, 30901],
                [118, 18472],
                [65, 1046],
                [203, 29817],
                [200, 30659],
                [201, 43444],
                [170, 30241],
                [14, 23252],
                [110, 33106],
                [135, 19006],
                [203, 29300],
                [90, 36559],
                [158, 586],
                [200, 37704],
                [111, 43526],
                [140, 41105],
                [92, 27777],
                [127, 36773],
                [51, 11152],
                [20, 29164],
                [150, 43544],
                [147, 44802],
                [73, 28341],
                [155, 17949],
                [68, 20940],
                [212, 6186],
                [198, 30348],
                [35, 13909],
                [44, 43975],
                [9, 19118],
                [89, 23777],
                [79, 16396],
                [60, 45824],
                [64, 19467],
                [35, 4119],
                [171, 1879],
                [165, 3387],
                [206, 2117],
                [81, 39846],
                [95, 22380],
                [13, 29349],
                [11, 17023],
                [66, 26353],
                [199, 29553],
                [211, 46645],
                [46, 14349],
                [102, 21830],
                [117, 41563],
                [197, 12282],
                [148, 23583],
                [18, 37877],
                [53, 763],
                [52, 41739],
                [145, 12824],
                [108, 12545],
                [150, 2872],
                [56, 23133],
                [215, 841],
                [106, 12642],
                [153, 31276],
                [144, 27749],
                [91, 24232],
                [140, 42303],
                [194, 24819],
                [86, 31768],
                [68, 35969],
                [89, 8403],
                [160, 32599],
                [128, 2156],
                [206, 27881],
                [147, 15127],
                [99, 23725],
                [7, 22780],
                [34, 20977],
                [48, 37100],
                [134, 1121],
                [189, 20631],
                [9, 39859],
                [119, 3113],
                [160, 32999],
                [10, 19797],
                [183, 20989],
                [83, 3501],
                [104, 26458],
                [87, 6438],
                [192, 26734],
                [112, 7856],
                [200, 5004],
                [203, 7518],
                [41, 22607],
                [139, 9825],
                [119, 31361],
                [18, 18878],
                [45, 12894],
                [41, 19610],
                [22, 27572],
                [128, 19888],
                [31, 27090],
                [32, 17307],
                [174, 40567],
                [107, 4829],
                [177, 45051],
                [89, 26341],
                [194, 39002],
                [181, 22322],
                [156, 24556],
                [148, 33654],
                [198, 27130],
                [179, 20045],
                [170, 41048],
                [5, 25666],
                [125, 20436],
                [205, 19926],
                [92, 16360],
                [149, 3639],
                [105, 40560],
                [186, 15608],
                [112, 9924],
                [186, 39069],
                [150, 29182],
                [153, 24719],
                [197, 34704],
                [171, 28461],
                [191, 27233],
                [192, 15704],
                [34, 38362],
                [101, 13161],
                [182, 2929],
                [188, 31968],
                [39, 20565],
                [155, 40662],
                [186, 44479],
                [53, 12865],
                [160, 32151],
                [216, 8172],
                [206, 45329],
                [78, 39028],
                [16, 12968],
                [153, 6550],
                [5, 15400],
                [180, 21293],
                [168, 44710],
                [60, 46095],
                [110, 10089],
                [86, 45214],
                [176, 46371],
                [32, 37813],
                [110, 8054],
                [49, 23163],
                [203, 24933],
                [106, 43514],
                [171, 376],
                [28, 31771],
                [43, 12032],
                [12, 5602],
                [125, 32421],
                [146, 41518],
                [143, 11747],
                [100, 10039],
                [13, 12202],
                [91, 6031],
                [64, 47000],
                [36, 23054],
                [141, 8399],
                [86, 10025],
                [111, 9153],
                [24, 36379],
                [155, 13826],
                [127, 20537],
                [11, 45051],
                [33, 43216],
                [208, 35223],
                [108, 7255],
                [164, 38419],
                [31, 39277],
                [24, 2002],
                [134, 42597],
                [155, 16313],
                [71, 15658],
                [178, 43703],
                [204, 24244],
                [47, 35058],
                [207, 27781],
                [116, 16053],
                [128, 39919],
                [199, 9331],
                [40, 28330],
                [87, 16860],
                [202, 1878],
                [119, 27656],
                [211, 36649],
                [195, 45395],
                [167, 11121],
                [108, 11828],
                [157, 25567],
                [129, 5427],
                [13, 16206],
                [200, 35545],
                [132, 39932],
                [83, 38602],
                [25, 26955],
                [138, 44636],
                [49, 9468],
                [128, 32117],
                [20, 6154],
                [182, 28545],
                [99, 10041],
                [129, 3029],
                [198, 40641],
                [143, 7602],
                [51, 6329],
                [204, 26471],
                [159, 25129],
                [175, 45832],
                [156, 34087],
                [107, 36031],
                [127, 8236],
                [188, 34233],
                [92, 15685],
                [71, 39464],
                [1, 37117],
                [172, 31827],
                [199, 32835],
                [9, 19238],
                [155, 16955],
                [207, 313],
                [4, 3604],
                [79, 41753],
                [190, 22402],
                [2, 14455],
                [154, 35507],
                [149, 14244],
                [2, 1048],
                [113, 24904],
                [44, 39134],
                [66, 7337],
                [126, 39349],
                [208, 2587],
                [124, 37064],
                [161, 47409],
                [69, 1077],
                [56, 25012],
                [100, 46468],
                [133, 2569],
                [213, 28037],
                [112, 34294],
                [108, 7616],
                [102, 22565],
                [75, 38401],
                [194, 17694],
                [132, 43653],
                [76, 39592],
                [40, 47486],
                [156, 34050],
                [115, 36336],
                [160, 9386],
                [168, 37827],
                [92, 42563],
                [145, 15188],
                [28, 43625],
                [143, 35976],
                [82, 3230],
                [176, 34026],
                [144, 16180],
                [181, 46227],
                [62, 29330],
                [10, 23409],
                [71, 79],
                [176, 29958],
                [127, 38930],
                [162, 15640],
                [210, 44282],
                [193, 13083],
                [91, 25426],
                [12, 22538],
            ],
            216,
            3,
            47520,
        ),
        857,
    );

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 1], [2, 1], [1, 1]] as &[_], 2, 3, 3), 4),
            ((&[[1, 2], [3, 3], [3, 1], [3, 1], [2, 4]], 3, 3, 6), 6),
            ((&[[1, 4], [1, 2], [2, 1], [2, 1], [3, 2], [3, 4]], 3, 6, 7), 6),
            ((&[[1, 1], [1, 1], [2, 1]], 2, 2, 2), 4),
            LONG_TEST_CASE_1,
            LONG_TEST_CASE_2,
            LONG_TEST_CASE_3,
        ];

        for ((boxes, ports_count, max_boxes, max_weight), expected) in test_cases {
            assert_eq!(
                S::box_delivering(
                    boxes.iter().map(Vec::from).collect(),
                    ports_count,
                    max_boxes,
                    max_weight,
                ),
                expected,
            );
        }
    }
}
