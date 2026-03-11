mod math {
    pub fn gcd<T: PartialOrd + Default + std::ops::Rem<Output = T> + Copy>(mut a: T, mut b: T) -> T {
        let zero = T::default();
        while a > zero {
            (a, b) = (b % a, a);
        }
        b
    }
    pub fn fpow<E: Default + std::ops::ShrAssign<usize> + PartialOrd>(mut a: i64, mut b: E, m: i64) -> i64 {
        let zero = E::default();
        let mut ret = 1_i64;
        while b > zero {
            ret = ret * a % m;
            a = a * a % m;
            b >>= 1;
        }
        ret
    }
}
