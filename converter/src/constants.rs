pub const CHAR_WIDTH: u32 = 9;
pub const CHAR_HEIGHT: u32 = 16;
pub const CHARS_PER_LINE: u32 = 32;
// pub const CHARS_PER_COLUMN: u32 = 8; // not needed rn

pub const VGA_WIDTH: u32 = 80;
pub const VGA_HEIGHT: u32 = 25;

pub const FOREGROUND: [[u8; 3]; 16] = [
    [0, 0, 0],
    [0, 0, 170],
    [0, 170, 0],
    [0, 170, 170],
    [170, 0, 0],
    [170, 0, 170],
    [170, 85, 0],
    [170, 170, 170],
    [85, 85, 85],
    [85, 85, 255],
    [85, 255, 85],
    [85, 255, 255],
    [255, 85, 85],
    [255, 85, 255],
    [255, 255, 85],
    [255, 255, 255],
];

pub const BACKGROUND: [[u8; 3]; 8] = [
    [0, 0, 0],
    [0, 0, 170],
    [0, 170, 0],
    [0, 170, 170],
    [170, 0, 0],
    [170, 0, 170],
    [170, 85, 0],
    [170, 170, 170],
];

pub const CHARACTERS: [[u16; 9]; 256] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2040, 2052, 2196, 2436, 2436, 2196, 2052, 2040, 0],
    [2040, 4092, 3948, 3708, 3708, 3948, 4092, 2040, 0],
    [480, 1008, 2032, 4064, 2032, 1008, 480, 0, 0],
    [128, 448, 992, 2032, 992, 448, 128, 0, 0],
    [448, 448, 2544, 3640, 3640, 2544, 448, 448, 0],
    [192, 480, 2544, 4088, 4088, 2544, 480, 192, 0],
    [0, 0, 384, 960, 960, 384, 0, 0, 0],
    [65535, 65535, 65151, 64575, 64575, 65151, 65535, 65535, 0],
    [0, 960, 1632, 1056, 1056, 1632, 960, 0, 0],
    [65535, 64575, 63903, 64479, 64479, 63903, 64575, 65535, 0],
    [1920, 4032, 2144, 2164, 4060, 1932, 60, 0, 0],
    [0, 632, 764, 3972, 3972, 764, 632, 0, 0],
    [3072, 3584, 4092, 2044, 20, 20, 28, 28, 0],
    [7168, 8188, 4092, 20, 20, 3604, 4092, 2044, 0],
    [672, 672, 448, 3960, 3960, 448, 672, 672, 0],
    [4094, 2044, 1016, 496, 224, 64, 64, 0, 0],
    [64, 64, 224, 496, 1016, 2044, 4094, 0, 0],
    [0, 272, 792, 2044, 2044, 792, 272, 0, 0],
    [0, 3580, 3580, 0, 0, 3580, 3580, 0, 0],
    [56, 124, 68, 4092, 4092, 4, 4092, 4092, 0],
    [2244, 6638, 4922, 4626, 5938, 7654, 2244, 0, 0],
    [3840, 3840, 3840, 3840, 3840, 3840, 3840, 0, 0],
    [0, 2320, 2840, 4092, 4092, 2840, 2320, 0, 0],
    [0, 16, 24, 4092, 4092, 24, 16, 0, 0],
    [0, 512, 1536, 4092, 4092, 1536, 512, 0, 0],
    [128, 128, 128, 672, 992, 448, 128, 0, 0],
    [128, 448, 992, 672, 128, 128, 128, 0, 0],
    [960, 960, 512, 512, 512, 512, 512, 0, 0],
    [128, 448, 992, 128, 992, 448, 128, 0, 0],
    [1536, 1920, 2016, 2032, 2016, 1920, 1536, 0, 0],
    [48, 240, 1008, 2032, 1008, 240, 48, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 56, 3580, 3580, 56, 0, 0, 0],
    [0, 14, 30, 0, 0, 30, 14, 0, 0],
    [544, 4088, 4088, 544, 4088, 4088, 544, 0, 0],
    [1592, 3196, 2116, 14407, 14407, 4044, 1944, 0, 0],
    [3120, 1584, 768, 384, 192, 3168, 3120, 0, 0],
    [1920, 4056, 2172, 2276, 1980, 4056, 2112, 0, 0],
    [0, 16, 30, 14, 0, 0, 0, 0, 0],
    [0, 0, 1008, 2040, 3084, 2052, 0, 0, 0],
    [0, 0, 2052, 3084, 2040, 1008, 0, 0, 0],
    [128, 672, 992, 448, 448, 992, 672, 128, 0],
    [0, 128, 128, 992, 992, 128, 128, 0, 0],
    [0, 0, 4096, 7680, 3584, 0, 0, 0, 0],
    [128, 128, 128, 128, 128, 128, 128, 0, 0],
    [0, 0, 0, 3072, 3072, 0, 0, 0, 0],
    [3072, 1536, 768, 384, 192, 96, 48, 0, 0],
    [1008, 2040, 3084, 2244, 3084, 2040, 1008, 0, 0],
    [0, 2064, 2072, 4092, 4092, 2048, 2048, 0, 0],
    [3592, 3852, 2436, 2244, 2148, 3132, 3096, 0, 0],
    [1032, 3084, 2116, 2116, 2116, 4092, 1976, 0, 0],
    [192, 224, 176, 2200, 4092, 4092, 2176, 0, 0],
    [1148, 3196, 2116, 2116, 2116, 4036, 1924, 0, 0],
    [2032, 4088, 2124, 2116, 2116, 4032, 1920, 0, 0],
    [12, 12, 3844, 3972, 196, 124, 60, 0, 0],
    [1976, 4092, 2116, 2116, 2116, 4092, 1976, 0, 0],
    [56, 2172, 2116, 2116, 3140, 2044, 1016, 0, 0],
    [0, 0, 0, 1584, 1584, 0, 0, 0, 0],
    [0, 0, 2048, 3632, 1584, 0, 0, 0, 0],
    [0, 128, 448, 864, 1584, 3096, 2056, 0, 0],
    [0, 288, 288, 288, 288, 288, 288, 0, 0],
    [0, 2056, 3096, 1584, 864, 448, 128, 0, 0],
    [24, 28, 4, 3524, 3556, 60, 24, 0, 0],
    [2032, 4088, 2056, 3016, 3016, 3064, 496, 0, 0],
    [4064, 4080, 152, 140, 152, 4080, 4064, 0, 0],
    [2052, 4092, 4092, 2116, 2116, 4092, 1976, 0, 0],
    [1008, 2040, 3084, 2052, 2052, 3084, 1560, 0, 0],
    [2052, 4092, 4092, 2052, 3084, 2040, 1008, 0, 0],
    [2052, 4092, 4092, 2116, 2276, 3084, 3612, 0, 0],
    [2052, 4092, 4092, 2116, 228, 12, 28, 0, 0],
    [1008, 2040, 3084, 2180, 2180, 1932, 3992, 0, 0],
    [4092, 4092, 64, 64, 64, 4092, 4092, 0, 0],
    [0, 0, 2052, 4092, 4092, 2052, 0, 0, 0],
    [1792, 3840, 2048, 2052, 4092, 2044, 4, 0, 0],
    [2052, 4092, 4092, 192, 480, 3900, 3612, 0, 0],
    [2052, 4092, 4092, 2052, 2048, 3072, 3584, 0, 0],
    [4092, 4092, 56, 112, 56, 4092, 4092, 0, 0],
    [4092, 4092, 56, 112, 224, 4092, 4092, 0, 0],
    [2040, 4092, 2052, 2052, 2052, 4092, 2040, 0, 0],
    [2052, 4092, 4092, 2116, 68, 124, 56, 0, 0],
    [2040, 4092, 2052, 3588, 15364, 16380, 10232, 0, 0],
    [2052, 4092, 4092, 68, 196, 4092, 3896, 0, 0],
    [1560, 3644, 2148, 2116, 2244, 3996, 1816, 0, 0],
    [0, 28, 2060, 4092, 4092, 2060, 28, 0, 0],
    [2044, 4092, 2048, 2048, 2048, 4092, 2044, 0, 0],
    [508, 1020, 1536, 3072, 1536, 1020, 508, 0, 0],
    [2044, 4092, 3584, 960, 3584, 4092, 2044, 0, 0],
    [3084, 3900, 1008, 480, 1008, 3900, 3084, 0, 0],
    [0, 60, 2172, 4032, 4032, 2172, 60, 0, 0],
    [3612, 3852, 2436, 2244, 2148, 3132, 3612, 0, 0],
    [0, 0, 4092, 4092, 2052, 2052, 0, 0, 0],
    [56, 112, 224, 448, 896, 1792, 3584, 0, 0],
    [0, 0, 2052, 2052, 4092, 4092, 0, 0, 0],
    [8, 12, 6, 3, 6, 12, 8, 0, 0],
    [8192, 8192, 8192, 8192, 8192, 8192, 8192, 8192, 0],
    [0, 0, 3, 7, 4, 0, 0, 0, 0],
    [1792, 4000, 2208, 2208, 2016, 4032, 2048, 0, 0],
    [4, 4092, 4092, 2080, 2144, 4032, 1920, 0, 0],
    [1984, 4064, 2080, 2080, 2080, 3168, 1088, 0, 0],
    [1920, 4032, 2144, 2084, 2044, 4092, 2048, 0, 0],
    [1984, 4064, 2208, 2208, 2208, 3296, 1216, 0, 0],
    [2112, 4088, 4092, 2116, 12, 24, 0, 0, 0],
    [10176, 28640, 18464, 18464, 32704, 16352, 32, 0, 0],
    [2052, 4092, 4092, 64, 32, 4064, 4032, 0, 0],
    [0, 0, 2080, 4076, 4076, 2048, 0, 0, 0],
    [0, 12288, 28672, 16384, 16416, 32748, 16364, 0, 0],
    [2052, 4092, 4092, 384, 960, 3680, 3104, 0, 0],
    [0, 0, 2052, 4092, 4092, 2048, 0, 0, 0],
    [4064, 4064, 96, 1984, 96, 4064, 4032, 0, 0],
    [32, 4064, 4032, 32, 32, 4064, 4032, 0, 0],
    [1984, 4064, 2080, 2080, 2080, 4064, 1984, 0, 0],
    [16416, 32736, 32704, 18464, 2080, 4064, 1984, 0, 0],
    [1984, 4064, 2080, 18464, 32704, 32736, 16416, 0, 0],
    [2080, 4064, 4032, 2144, 32, 224, 192, 0, 0],
    [1088, 3296, 2464, 2336, 2848, 3680, 1088, 0, 0],
    [32, 32, 2040, 4092, 2080, 3104, 1024, 0, 0],
    [2016, 4064, 2048, 2048, 2016, 4064, 2048, 0, 0],
    [0, 992, 2016, 3072, 3072, 2016, 992, 0, 0],
    [2016, 4064, 3072, 1920, 3072, 4064, 2016, 0, 0],
    [2080, 3168, 1984, 896, 1984, 3168, 2080, 0, 0],
    [18400, 20448, 18432, 18432, 26624, 16352, 8160, 0, 0],
    [3168, 3680, 2848, 2464, 2272, 3168, 3104, 0, 0],
    [0, 64, 64, 2040, 4028, 2052, 2052, 0, 0],
    [0, 0, 0, 4028, 4028, 0, 0, 0, 0],
    [0, 2052, 2052, 4028, 2040, 64, 64, 0, 0],
    [8, 12, 4, 12, 8, 12, 4, 0, 0],
    [1920, 1984, 1120, 1072, 1120, 1984, 1920, 0, 0],
    [4064, 4080, 152, 140, 152, 4080, 4064, 0, 0],
    [2052, 4092, 4092, 2116, 2116, 4092, 1976, 0, 0],
    [4092, 4092, 4, 4, 4, 4, 12, 0, 0],
    [4064, 4080, 2072, 2060, 2072, 4080, 4064, 0, 0],
    [2052, 4092, 4092, 2116, 2276, 3084, 3612, 0, 0],
    [3612, 3852, 2436, 2244, 2148, 3132, 3612, 0, 0],
    [4092, 4092, 64, 64, 64, 4092, 4092, 0, 0],
    [2040, 4092, 2116, 2116, 2116, 4092, 2040, 0, 0],
    [0, 0, 2052, 4092, 4092, 2052, 0, 0, 0],
    [2052, 4092, 4092, 192, 480, 3900, 3612, 0, 0],
    [4064, 4080, 24, 12, 24, 4080, 4064, 0, 0],
    [4092, 4092, 56, 112, 56, 4092, 4092, 0, 0],
    [4092, 4092, 56, 112, 224, 4092, 4092, 0, 0],
    [3084, 2116, 2116, 2116, 2116, 2116, 3084, 0, 0],
    [2040, 4092, 2052, 2052, 2052, 4092, 2040, 0, 0],
    [4092, 4092, 4, 4, 4, 4092, 4092, 0, 0],
    [2052, 4092, 4092, 2116, 68, 124, 56, 0, 0],
    [3084, 3612, 2868, 2532, 2244, 3084, 3612, 0, 0],
    [0, 28, 2060, 4092, 4092, 2060, 28, 0, 0],
    [0, 60, 2172, 4032, 4032, 2172, 60, 0, 0],
    [248, 2556, 2308, 4092, 2308, 2556, 248, 0, 0],
    [3084, 3900, 1008, 480, 1008, 3900, 3084, 0, 0],
    [252, 2556, 2304, 4092, 2304, 2556, 252, 0, 0],
    [2552, 4092, 3844, 4, 3844, 4092, 2552, 0, 0],
    [1984, 4064, 2080, 2080, 1984, 4064, 2080, 0, 0],
    [32760, 32764, 2116, 2116, 4092, 1976, 0, 0, 0],
    [12768, 31712, 19968, 17408, 19968, 31712, 12768, 0, 0],
    [1804, 3996, 2228, 2276, 2244, 3972, 1792, 0, 0],
    [1728, 4064, 2336, 2336, 2336, 2336, 2080, 0, 0],
    [1924, 4036, 18532, 18484, 18460, 30732, 12292, 0, 0],
    [32, 4064, 4032, 32, 32, 4064, 4032, 0, 0],
    [1928, 4044, 2116, 2116, 4092, 2040, 0, 0, 0],
    [0, 0, 0, 2016, 4064, 2048, 2048, 0, 0],
    [2080, 4064, 4064, 384, 960, 3680, 3104, 0, 0],
    [3848, 3980, 196, 68, 68, 4092, 4088, 0, 0],
    [32736, 32736, 2048, 2048, 2016, 4064, 2048, 0, 0],
    [0, 992, 2016, 3072, 3072, 2016, 992, 0, 0],
    [1028, 3780, 19428, 18740, 18716, 30988, 12292, 0, 0],
    [1984, 4064, 2080, 2080, 2080, 4064, 1984, 0, 0],
    [4064, 4064, 32, 32, 32, 4064, 4064, 0, 0],
    [0, 32704, 32736, 2080, 2080, 4064, 1984, 0, 0],
    [1984, 4064, 2080, 2080, 4064, 2016, 32, 0, 0],
    [1216, 3552, 2336, 2336, 2336, 3936, 1600, 0, 0],
    [32, 32, 2016, 4064, 2080, 3104, 1024, 0, 0],
    [2016, 4064, 2048, 2048, 2048, 4064, 2016, 0, 0],
    [1984, 4064, 2080, 32672, 2208, 4000, 1792, 0, 0],
    [2080, 3168, 1984, 896, 1984, 3168, 2080, 0, 0],
    [2016, 4064, 2048, 32736, 2048, 4064, 2016, 0, 0],
    [0, 21845, 0, 43690, 0, 21845, 0, 43690, 43690],
    [
        43690, 21845, 43690, 21845, 43690, 21845, 43690, 21845, 21845,
    ],
    [
        21845, 65535, 43690, 65535, 21845, 65535, 43690, 65535, 65535,
    ],
    [0, 0, 0, 65535, 65535, 0, 0, 0, 0],
    [128, 128, 128, 65535, 65535, 0, 0, 0, 0],
    [160, 160, 160, 65535, 65535, 0, 0, 0, 0],
    [128, 128, 65535, 65535, 0, 65535, 65535, 0, 0],
    [128, 128, 65408, 65408, 128, 65408, 65408, 0, 0],
    [160, 160, 160, 65504, 65504, 0, 0, 0, 0],
    [160, 160, 65471, 65471, 0, 65535, 65535, 0, 0],
    [0, 0, 65535, 65535, 0, 65535, 65535, 0, 0],
    [160, 160, 65440, 65440, 32, 65504, 65504, 0, 0],
    [160, 160, 191, 191, 128, 255, 255, 0, 0],
    [128, 128, 255, 255, 128, 255, 255, 0, 0],
    [160, 160, 160, 255, 255, 0, 0, 0, 0],
    [128, 128, 128, 65408, 65408, 0, 0, 0, 0],
    [0, 0, 0, 255, 255, 128, 128, 128, 128],
    [128, 128, 128, 255, 255, 128, 128, 128, 128],
    [128, 128, 128, 65408, 65408, 128, 128, 128, 128],
    [0, 0, 0, 65535, 65535, 128, 128, 128, 128],
    [128, 128, 128, 128, 128, 128, 128, 128, 128],
    [128, 128, 128, 65535, 65535, 128, 128, 128, 128],
    [0, 0, 0, 65535, 65535, 160, 160, 160, 160],
    [0, 0, 65535, 65535, 0, 65535, 65535, 128, 128],
    [0, 0, 255, 255, 128, 191, 191, 160, 160],
    [0, 0, 65504, 65504, 32, 65440, 65440, 160, 160],
    [160, 160, 191, 191, 128, 191, 191, 160, 160],
    [160, 160, 65440, 65440, 32, 65440, 65440, 160, 160],
    [0, 0, 65535, 65535, 0, 65471, 65471, 160, 160],
    [160, 160, 160, 160, 160, 160, 160, 160, 160],
    [160, 160, 65471, 65471, 0, 65471, 65471, 160, 160],
    [160, 160, 160, 191, 191, 160, 160, 160, 160],
    [128, 128, 255, 255, 128, 255, 255, 128, 128],
    [160, 160, 160, 65440, 65440, 160, 160, 160, 160],
    [128, 128, 65408, 65408, 128, 65408, 65408, 128, 128],
    [0, 0, 255, 255, 128, 255, 255, 128, 128],
    [0, 0, 0, 255, 255, 160, 160, 160, 160],
    [0, 0, 0, 65504, 65504, 160, 160, 160, 160],
    [0, 0, 65408, 65408, 128, 65408, 65408, 128, 128],
    [128, 128, 65535, 65535, 128, 65535, 65535, 128, 128],
    [160, 160, 160, 65535, 65535, 160, 160, 160, 160],
    [128, 128, 128, 255, 255, 0, 0, 0, 0],
    [0, 0, 0, 65408, 65408, 128, 128, 128, 128],
    [
        65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
    ],
    [
        65408, 65408, 65408, 65408, 65408, 65408, 65408, 65408, 65408,
    ],
    [65535, 65535, 65535, 65535, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 65535, 65535, 65535, 65535, 65535],
    [127, 127, 127, 127, 127, 127, 127, 127, 127],
    [1984, 4064, 2080, 1920, 2080, 4064, 1984, 0, 0],
    [1984, 4064, 2088, 2092, 1990, 4066, 2080, 0, 0],
    [1728, 4064, 2336, 2344, 2348, 2342, 2082, 0, 0],
    [32, 4064, 4032, 40, 44, 4070, 4034, 0, 0],
    [0, 12, 12, 2016, 4064, 2060, 2060, 0, 0],
    [0, 0, 0, 2024, 4076, 2054, 2050, 0, 0],
    [1984, 4064, 2080, 2088, 2092, 4070, 1986, 0, 0],
    [2016, 4064, 2048, 2056, 2060, 4070, 2018, 0, 0],
    [2028, 4076, 2048, 2048, 2048, 4076, 2028, 0, 0],
    [1984, 4064, 2080, 1928, 2092, 4070, 1986, 0, 0],
    [4070, 4083, 153, 140, 152, 4080, 4064, 0, 0],
    [6, 3, 4093, 4092, 2116, 2116, 3148, 0, 0],
    [4092, 4092, 64, 71, 67, 4092, 4092, 0, 0],
    [6, 3, 2053, 4092, 4092, 2052, 0, 0, 0],
    [6, 2043, 4093, 2052, 2052, 4092, 2040, 0, 0],
    [0, 60, 2172, 4034, 4035, 2173, 60, 0, 0],
    [6, 2299, 2557, 2308, 2308, 2556, 2296, 0, 0],
    [2048, 2112, 2112, 2544, 2544, 2112, 2112, 2048, 0],
    [0, 2048, 2568, 2840, 2480, 2272, 2112, 0, 0],
    [0, 2112, 2272, 2480, 2840, 2568, 2048, 0, 0],
    [3, 3, 2052, 4092, 4092, 2051, 3, 0, 0],
    [3, 59, 2168, 4032, 4032, 2171, 59, 0, 0],
    [0, 128, 128, 1712, 1712, 128, 128, 0, 0],
    [576, 864, 288, 864, 576, 864, 288, 0, 0],
    [0, 12, 30, 18, 30, 12, 0, 0, 0],
    [0, 0, 0, 384, 384, 0, 0, 0, 0],
    [0, 0, 0, 256, 256, 0, 0, 0, 0],
    [128, 896, 1920, 3072, 4094, 4094, 2, 2, 0],
    [2, 126, 124, 2, 126, 124, 0, 0, 0],
    [100, 118, 90, 78, 100, 0, 0, 0, 0],
    [0, 2032, 2032, 2032, 2032, 2032, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
];