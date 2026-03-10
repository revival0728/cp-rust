mod segtree {
    pub trait SingleSeg<V: Copy> {
        fn seg_len(len: usize) -> usize { 4 * len + 5 }
        fn len(&self) -> usize;
        fn bid() -> usize { 1 }
        fn lc(id: usize) -> usize { id << 1 }
        fn rc(id: usize) -> usize { id << 1 | 1 }

        fn dummy() -> V;
        fn merge(lv: V, rv: V) -> V;
        fn init<C: std::ops::Index<usize>>(&mut self, seg_id: usize, arr_id: usize, arr: &mut C) -> ();
        fn get(&self, id: usize) -> V;
        fn set(&mut self, id: usize, val: V) -> ();
        fn upd(&mut self, id: usize, val: V) -> ();

        fn pull(&mut self, id: usize, l: usize, r: usize) {
            if r - l <= 1 { return; }
            let lc = Self::lc(id);
            let rc = Self::rc(id);
            let cv = Self::merge(self.get(lc), self.get(rc));
            self.set(id, cv);
        }
        fn inner_build<C: std::ops::Index<usize>>(&mut self, id: usize, l: usize, r: usize, arr: &mut C) {
            if r - l == 1 {
                self.init(id, l, arr);
                return;
            }
            let m = (l + r) >> 1;
            let lc = Self::lc(id);
            let rc = Self::rc(id);
            self.inner_build(lc, l, m, arr);
            self.inner_build(rc, m, r, arr);
            self.pull(id, l, r);
        }
        fn inner_update(&mut self, id: usize, l: usize, r: usize, x: usize, val: V) {
            if r - l == 1 {
                self.upd(id, val);
                return;
            }
            let m = (l + r) >> 1;
            let lc = Self::lc(id);
            let rc = Self::rc(id);
            if x < m { self.inner_update(lc, l, m, x, val); }
            else     { self.inner_update(rc, m, r, x, val); }
            self.pull(id, l, r);
        }
        fn inner_query(&self, id: usize, l: usize, r: usize, ql: usize, qr: usize) -> V {
            if ql <= l && r <= qr { return self.get(id); }
            if r <= ql || l >= qr { return Self::dummy(); }
            let m = (l + r) >> 1;
            let lc = Self::lc(id);
            let rc = Self::rc(id);
            Self::merge(
                self.inner_query(lc, l, m, ql, qr),
                self.inner_query(rc, m, r, ql, qr)
            )
        }
        fn build<C: std::ops::Index<usize>>(&mut self, arr: &mut C) { self.inner_build(Self::bid(), 1, self.len(), arr); }
        fn update(&mut self, x: usize, val: V) { self.inner_update(Self::bid(), 1, self.len(), x, val); }
        fn query(&mut self, ql: usize, qr: usize) { self.inner_query(Self::bid(), 1, self.len(), ql, qr); }
    }

    pub trait RangeSeg<V: Copy, T: Copy>: SingleSeg<V> {
        fn dummy_tag() -> T;
        fn is_tagged(tag: T) -> bool;
        fn get_tag(&self, id: usize) -> T;
        fn set_tag(&mut self, id: usize, tag: T) -> ();
        fn upd_tag(&mut self, id: usize, tag: T) -> ();
        fn upd_by_tag(&mut self, id: usize, l: usize, r: usize) -> ();

        fn push(&mut self, id: usize, l: usize, r: usize) {
            if r - l <= 1 { return; }
            let lc = Self::lc(id);
            let rc = Self::rc(id);
            let tag = self.get_tag(id);
            if Self::is_tagged(tag) {
                self.upd_by_tag(id, l, r);
                if r - l > 1 {
                    self.upd_tag(lc, tag);
                    self.upd_tag(rc, tag);
                }
                self.set_tag(id, Self::dummy_tag());
            }
        }
        fn inner_update(&mut self, id: usize, l: usize, r: usize, ql: usize, qr: usize, tag: T) {
            self.push(id, l, r);
            if ql <= l && r <= qr {
                self.upd_tag(id, tag);
                self.push(id, l, r);
                return;
            }
            if r <= ql || l >= qr { return; }
            let m = (l + r) >> 1;
            let lc = Self::lc(id);
            let rc = Self::rc(id);
            <Self as RangeSeg<V, T>>::inner_update(self, lc, l, m, ql, qr, tag);
            <Self as RangeSeg<V, T>>::inner_update(self, rc, m, r, ql, qr, tag);
            self.pull(id, l, r);
        }
        fn range_update(&mut self, ql: usize, qr: usize, tag: T) { <Self as RangeSeg<V, T>>::inner_update(self, Self::bid(), 1, self.len(), ql, qr, tag); }
    }
}