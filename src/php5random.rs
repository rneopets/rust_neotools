use pyo3::prelude::*;

pub struct Php5Random {
    r: Vec<u32>,
    k: usize,
}

impl Php5Random {
    pub fn new(seed: u32) -> Php5Random {
        let mut phpr = Php5Random {
            r: vec![0; 34],
            k: 0,
        };
        phpr.srand(seed);
        phpr
    }

    pub fn srand(&mut self, seed: u32) {
        self.r = vec![0; 34];
        self.r[0] = seed;

        for i in 1..31 {
            self.r[i] = ((16807_u64 * self.r[i - 1] as u64) % 2147483647) as u32;
        }

        for i in 31..34 {
            self.r[i] = self.r[i - 31];
        }

        self.k = 0;

        for _ in 0..310 {
            _ = &self.rand();
        }
    }

    #[inline]
    pub fn rand(&mut self) -> u32 {
        let k_as_isize = self.k as isize;
        self.r[self.k] = (self.r[(k_as_isize - 31).rem_euclid(34) as usize] as i64
            + self.r[(k_as_isize - 3).rem_euclid(34) as usize] as i64)
            as u32;
        let r = self.r[self.k] >> 1;
        self.k = (self.k + 1) % 34;
        r
    }

    pub fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        let r = self.rand();
        (min as f64 + ((max as f64 - min as f64 + 1.0) * (r as f64 / 2147483647_f64))) as u32
    }
}

const MT_N: usize = 624;
const MT_M: usize = 397;
const MT_MATRIX_A: u32 = 0x9908b0df;
const MT_UPPER_MASK: u32 = 0x80000000;
const MT_LOWER_MASK: u32 = 0x7fffffff;

/// PHP5's `mt_rand()`/`mt_srand()` - a genuine Mersenne Twister (MT19937),
/// entirely separate from `Php5Random` above (which reimplements PHP5's
/// plain `rand()`/`srand()`, backed on Linux by glibc's `random()`).
pub struct Php5MtRandom {
    mt: Vec<u32>,
    idx: usize,
}

impl Php5MtRandom {
    pub fn new(seed: u32) -> Php5MtRandom {
        let mut mtr = Php5MtRandom {
            mt: vec![0; MT_N],
            idx: MT_N,
        };
        mtr.srand(seed);
        mtr
    }

    pub fn srand(&mut self, seed: u32) {
        self.mt[0] = seed;
        for i in 1..MT_N {
            self.mt[i] = 1812433253_u32
                .wrapping_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        self.reload();
    }

    // PHP's `twist(m, u, v)` macro picks the matrix-A constant off the low
    // bit of `u` (the pre-reload value at the current position), not `v`
    // (the next position) as in the textbook reference MT19937 twist step.
    fn twist(m: u32, u: u32, v: u32) -> u32 {
        let mixed = (u & MT_UPPER_MASK) | (v & MT_LOWER_MASK);
        let matrix_bit = if u & 1 == 1 { MT_MATRIX_A } else { 0 };
        m ^ (mixed >> 1) ^ matrix_bit
    }

    fn reload(&mut self) {
        let mut p = 0;
        for _ in 0..(MT_N - MT_M) {
            self.mt[p] = Self::twist(self.mt[p + MT_M], self.mt[p], self.mt[p + 1]);
            p += 1;
        }
        for _ in 0..(MT_M - 1) {
            self.mt[p] = Self::twist(self.mt[p + MT_M - MT_N], self.mt[p], self.mt[p + 1]);
            p += 1;
        }
        self.mt[p] = Self::twist(self.mt[p + MT_M - MT_N], self.mt[p], self.mt[0]);
        self.idx = 0;
    }

    #[inline]
    pub fn rand(&mut self) -> u32 {
        if self.idx >= MT_N {
            self.reload();
        }
        let mut y = self.mt[self.idx];
        self.idx += 1;

        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;

        // PHP's mt_rand() right-shifts off the low bit to keep results
        // within a signed 31-bit range, unlike the raw 32-bit MT19937 word.
        y >> 1
    }

    pub fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        let n = self.rand();
        (min as f64 + ((max as f64 - min as f64 + 1.0) * (n as f64 / 2147483648_f64))) as u32
    }
}

/// Python-exposed wrapper around `Php5Random` (PHP5's `rand()`/`srand()`).
#[pyclass(name = "Php5Random")]
pub struct PyPhp5Random(Php5Random);

#[pymethods]
impl PyPhp5Random {
    #[new]
    fn new(seed: u32) -> Self {
        PyPhp5Random(Php5Random::new(seed))
    }

    fn rand(&mut self) -> u32 {
        self.0.rand()
    }

    fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        self.0.rand_range(min, max)
    }
}

/// Python-exposed wrapper around `Php5MtRandom` (PHP5's `mt_rand()`/`mt_srand()`).
#[pyclass(name = "Php5MtRandom")]
pub struct PyPhp5MtRandom(Php5MtRandom);

#[pymethods]
impl PyPhp5MtRandom {
    #[new]
    fn new(seed: u32) -> Self {
        PyPhp5MtRandom(Php5MtRandom::new(seed))
    }

    fn rand(&mut self) -> u32 {
        self.0.rand()
    }

    fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        self.0.rand_range(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_seed_rand() {
        // all random numbers generated with seed 0 are 0, let's make sure
        let mut phpr = Php5Random::new(0);
        for _ in 0..100 {
            assert_eq!(phpr.rand(), 0);
        }
    }

    #[test]
    fn one_seed_rand() {
        // all random numbers generated with seed 1 are expected to be these values
        let mut phpr = Php5Random::new(1);
        let mut v = Vec::new();
        for _ in 0..100 {
            v.push(phpr.rand());
        }

        assert_eq!(
            v,
            vec![
                1804289383, 846930886, 1681692777, 1714636915, 1957747793, 424238335, 719885386,
                1649760492, 596516649, 1189641421, 1025202362, 1350490027, 783368690, 1102520059,
                2044897763, 1967513926, 1365180540, 1540383426, 304089172, 1303455736, 35005211,
                521595368, 294702567, 1726956429, 336465782, 861021530, 278722862, 233665123,
                2145174067, 468703135, 1101513929, 1801979802, 1315634022, 635723058, 1369133069,
                1125898167, 1059961393, 2089018456, 628175011, 1656478042, 1131176229, 1653377373,
                859484421, 1914544919, 608413784, 756898537, 1734575198, 1973594324, 149798315,
                2038664370, 1129566413, 184803526, 412776091, 1424268980, 1911759956, 749241873,
                137806862, 42999170, 982906996, 135497281, 511702305, 2084420925, 1937477084,
                1827336327, 572660336, 1159126505, 805750846, 1632621729, 1100661313, 1433925857,
                1141616124, 84353895, 939819582, 2001100545, 1998898814, 1548233367, 610515434,
                1585990364, 1374344043, 760313750, 1477171087, 356426808, 945117276, 1889947178,
                1780695788, 709393584, 491705403, 1918502651, 752392754, 1474612399, 2053999932,
                1264095060, 1411549676, 1843993368, 943947739, 1984210012, 855636226, 1749698586,
                1469348094, 1956297539
            ]
        );
    }

    #[test]
    fn one_seed_rand_range() {
        let mut phpr = Php5Random::new(1);
        let mut v = Vec::new();
        for _ in 0..100 {
            v.push(phpr.rand_range(0, 100));
        }
        assert_eq!(
            v,
            [
                84, 39, 79, 80, 92, 19, 33, 77, 28, 55, 48, 63, 36, 51, 96, 92, 64, 72, 14, 61, 1,
                24, 13, 81, 15, 40, 13, 10, 100, 22, 51, 84, 61, 29, 64, 52, 49, 98, 29, 77, 53,
                77, 40, 90, 28, 35, 81, 92, 7, 95, 53, 8, 19, 66, 89, 35, 6, 2, 46, 6, 24, 98, 91,
                85, 26, 54, 37, 76, 51, 67, 53, 3, 44, 94, 94, 72, 28, 74, 64, 35, 69, 16, 44, 88,
                83, 33, 23, 90, 35, 69, 96, 59, 66, 86, 44, 93, 40, 82, 69, 92
            ]
        );
    }

    // Golden vectors below were captured from a real `php5.6-cli` binary
    // (`mt_srand($seed); for(...) { echo mt_rand(), ","; }`), not hand
    // derived, so they double as a cross-check against real PHP5 behavior.

    #[test]
    fn mt_zero_seed_rand() {
        // unlike Php5Random, seed 0 has no all-zero special case for mt_rand
        let mut phpr = Php5MtRandom::new(0);
        let mut v = Vec::new();
        for _ in 0..100 {
            v.push(phpr.rand());
        }

        assert_eq!(
            v,
            vec![
                963932192, 1273124119, 1535857466, 324735766, 1294424481, 1842424189, 1170127713,
                1819459251, 909791748, 1339092841, 770137596, 1316527631, 1195978468, 638950699,
                225340245, 121790188, 68335706, 1571656624, 1314875883, 1025778016, 1700216563,
                1744119059, 1135793195, 1109659937, 1219866412, 843498920, 154273316, 1795465484,
                1985757392, 724552740, 1953298674, 767344316, 2111664463, 1349090462, 366525455,
                97516052, 483481225, 1857877713, 269974433, 269808022, 34121951, 1138017607,
                1716179948, 1719942744, 991019385, 1037366520, 480482367, 699298623, 253992391,
                594953637, 782441446, 892627606, 307848836, 998461482, 127999049, 508666982,
                1120660751, 227434853, 890479738, 1138033981, 568128849, 1754417545, 1662654181,
                555262159, 1160304429, 1694243192, 931759335, 290378816, 40350784, 1446413988,
                809851756, 321424322, 823317093, 1665069954, 1324855674, 829978760, 2026683559,
                1938315458, 1464197940, 966260245, 772037341, 825436281, 1220238538, 214687222,
                1498151584, 1946075781, 129333204, 57229674, 1431870609, 751952581, 714900301,
                367025541, 1705387425, 1385962335, 1875598683, 526225078, 677377223, 847679559,
                781062938, 1456525488
            ]
        );
    }

    #[test]
    fn mt_one_seed_rand() {
        let mut phpr = Php5MtRandom::new(1);
        let mut v = Vec::new();
        for _ in 0..100 {
            v.push(phpr.rand());
        }

        assert_eq!(
            v,
            vec![
                1244335972, 15217923, 1546885062, 2002651684, 2135443977, 1865258162, 1509498899,
                2145423170, 1837306065, 1634983062, 1956263230, 1300815158, 399990758, 1324153429,
                1399889920, 720488840, 852051651, 2009054860, 1002178351, 1817438858, 900213375,
                672749746, 687780031, 1126458602, 439057861, 1200153864, 1885742837, 1646869560,
                2093652664, 992762289, 714740847, 1962718498, 896155243, 1171151517, 952684674,
                1214960654, 1857275789, 2016761583, 1712363206, 485078877, 422918448, 1537534964,
                2079325892, 428554101, 1465236969, 199288222, 1486751499, 1112724124, 1882031360,
                1857616832, 1921153182, 1780579432, 1974030322, 360416282, 2075413040, 586370361,
                364708055, 127223797, 254610506, 1439948004, 1943884921, 883058517, 1252863927,
                716388245, 2057052106, 1274970734, 997537770, 1711451255, 667195540, 621974924,
                1462314382, 305200661, 680834640, 453530491, 362739436, 885920424, 2099032128,
                73382329, 525291490, 1340094185, 2123563015, 734282582, 1606673506, 1501490554,
                1554932673, 958066545, 1694964459, 477008835, 1920825339, 157548364, 1176462914,
                1130100605, 185020146, 1929685527, 630531571, 216684361, 1537091581, 256603838,
                279234226, 1126998093
            ]
        );
    }

    #[test]
    fn mt_one_seed_rand_range() {
        let mut phpr = Php5MtRandom::new(1);
        let mut v = Vec::new();
        for _ in 0..100 {
            v.push(phpr.rand_range(0, 100));
        }

        assert_eq!(
            v,
            [
                58, 0, 72, 94, 100, 87, 70, 100, 86, 76, 92, 61, 18, 62, 65, 33, 40, 94, 47, 85,
                42, 31, 32, 52, 20, 56, 88, 77, 98, 46, 33, 92, 42, 55, 44, 57, 87, 94, 80, 22, 19,
                72, 97, 20, 68, 9, 69, 52, 88, 87, 90, 83, 92, 16, 97, 27, 17, 5, 11, 67, 91, 41,
                58, 33, 96, 59, 46, 80, 31, 29, 68, 14, 32, 21, 17, 41, 98, 3, 24, 63, 99, 34, 75,
                70, 73, 45, 79, 22, 90, 7, 55, 53, 8, 90, 29, 10, 72, 12, 13, 53
            ]
        );
    }

    #[test]
    fn mt_seed_42_rand() {
        let mut phpr = Php5MtRandom::new(42);
        let mut v = Vec::new();
        for _ in 0..20 {
            v.push(phpr.rand());
        }

        assert_eq!(
            v,
            vec![
                1354439493, 1710563033, 2041643438, 1748058097, 586813251, 478617429, 871047720,
                858678140, 335047475, 957418556, 1824282015, 1937772317, 124733605, 1166237587,
                294988125, 1442644940, 1290884657, 306804147, 636086845, 1397772353
            ]
        );
    }

    #[test]
    fn mt_seed_42_range() {
        // Verified against a real php5.6-cli binary:
        // mt_srand(42); mt_rand(0,23).",".mt_rand(0,59) === "15,47"
        let mut phpr = Php5MtRandom::new(42);
        assert_eq!(phpr.rand_range(0, 23), 15);
        assert_eq!(phpr.rand_range(0, 59), 47);
    }
}
