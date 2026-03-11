mod fenwick {
    trait Fenwick<'a, T: 'a + Copy + Default> {
        fn data(&mut self) -> &'a mut Vec<T>;
        fn add(a: T, b: T) -> T;
        fn sub(a: T, b: T) -> T;

        fn build(bit: &mut Vec<T>) {
            let n = bit.len();
            for i in 1..n {
                let j = i + (1 << i.trailing_zeros());
                if j < n {
                    bit[j] = Self::add(bit[j], bit[i]);
                }
            }
        }
        fn update(&mut self, mut x: usize, v: T) {
            let bit = self.data();
            let n = bit.len();
            while x < n {
                bit[x] = Self::add(bit[x], v);
                x += 1 << x.trailing_zeros();
            }
        }
        fn sum(&mut self, mut x: usize) -> T {
            let bit = self.data();
            let mut ret = T::default();
            loop {
                ret = Self::add(ret, bit[x]);
                if let Some(v) = x.checked_sub(1 << x.trailing_zeros()) {
                    if v == 0 { break; }
                    x = v;
                } else {
                    break;
                }
            }
            ret
        }
        fn query(&mut self, ql: usize, qr: usize) -> T {
            Self::sub(self.sum(qr), self.sum(ql - 1))
        }
    }
}
