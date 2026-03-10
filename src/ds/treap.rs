mod treap {
    use rand::prelude::*;

    #[derive(Debug)]
    struct Treap<T: PartialOrd + Copy> {
        pub treap: inner::Ptr<inner::Treap<T>>,
        rng: ThreadRng,
    }

    impl<T: PartialOrd + Copy> Treap<T> {
        pub fn new(tree_key: T) -> Self { 
            let mut rng = rand::thread_rng();
            Self {
                treap: inner::Treap::new_ptr(tree_key, &mut rng),
                rng,
            }
        }
        pub fn new_inner(&mut self, tree_key: T) -> inner::Treap<T> {
            inner::Treap::new(tree_key, &mut self.rng)
        }
        pub fn insert(&mut self, tree_key: T) {
            self.treap = inner::Treap::insert(&mut self.treap, tree_key, &mut self.rng);
        }
        pub fn erase(&mut self, tree_key: T) {
            self.treap = inner::Treap::erase(&mut self.treap, tree_key);
        }
        pub fn get_by_rank(&self, rank: usize) -> T {
            inner::Treap::get_by_rank(&self.treap, rank)
        }
    }

    mod inner {
        use rand::prelude::*;
        pub type Ptr<T> = Option<Box<T>>;
        macro_rules! get { ($ptr:expr) => ($ptr.as_ref().unwrap()) }
        macro_rules! get_mut { ($ptr:expr) => ($ptr.as_mut().unwrap()) }
        macro_rules! new_treap { ($val:expr, $rng:ident) => (Some(Box::new(Treap::new($val, &mut $rng)))) }
        #[derive(Debug)]
        pub struct Treap<T: PartialOrd + Copy> {
            tree_key: T,
            heap_key: i64,
            size: usize,
            count: usize,
            lc: Ptr<Treap<T>>,
            rc: Ptr<Treap<T>>,
        }
        impl<T: PartialOrd + Copy> Treap<T> {
            pub fn new(tree_key: T, rng: &mut ThreadRng) -> Treap<T> {
                Treap {
                    tree_key,
                    heap_key: rng.r#gen(),
                    size: 1,  // size of treap
                    count: 1,   // count of same tree_key
                    lc: None,
                    rc: None,
                }
            }
            pub fn new_ptr(tree_key: T, rng: &mut ThreadRng) -> Ptr<Treap<T>> {
                Some(Box::new(Treap::new(tree_key, rng)))
            }
            pub fn get_size(tree: &Ptr<Treap<T>>) -> usize {
                if tree.is_none() { 0 } else { get!(tree).size }
            }
            pub fn upd_size(&mut self) {
                self.size = Treap::get_size(&self.lc) + Treap::get_size(&self.rc) + self.count;
            }
            pub fn merge(a: &mut Ptr<Treap<T>>, b: &mut Ptr<Treap<T>>) -> Ptr<Treap<T>> {
                let mut a = a.take();
                let mut b = b.take();
                if a.is_none() { return b; }
                if b.is_none() { return a; }
                if get!(a).heap_key < get!(b).heap_key {
                    get_mut!(a).rc = Treap::merge(&mut get_mut!(a).rc, &mut b);
                    get_mut!(a).upd_size();
                    return a;
                } else {
                    get_mut!(b).lc = Treap::merge(&mut a, &mut get_mut!(b).lc);
                    get_mut!(b).upd_size();
                    return b;
                }
            }
            pub fn split(cur: &mut Ptr<Treap<T>>, tree_key: T) -> (Ptr<Treap<T>>, Ptr<Treap<T>>) {
                let mut cur = cur.take();
                if cur.is_none() { return (None, None); }
                if get!(cur).tree_key < tree_key {
                    let (sm, lg) = Treap::split(&mut get_mut!(cur).rc, tree_key);
                    get_mut!(cur).rc = sm;
                    get_mut!(cur).upd_size();
                    return (cur, lg);
                } else {
                    let (sm, lg) = Treap::split(&mut get_mut!(cur).lc, tree_key);
                    get_mut!(cur).lc = lg;
                    get_mut!(cur).upd_size();
                    return (sm, cur);
                }
            }
            pub fn split_eq(cur: &mut Ptr<Treap<T>>, tree_key: T) -> (Ptr<Treap<T>>, Ptr<Treap<T>>) {
                let mut cur = cur.take();
                if cur.is_none() { return (None, None); }
                if get!(cur).tree_key <= tree_key {
                    let (sm, lg) = Treap::split(&mut get_mut!(cur).rc, tree_key);
                    get_mut!(cur).rc = sm;
                    get_mut!(cur).upd_size();
                    return (cur, lg);
                } else {
                    let (sm, lg) = Treap::split(&mut get_mut!(cur).lc, tree_key);
                    get_mut!(cur).lc = lg;
                    get_mut!(cur).upd_size();
                    return (sm, cur);
                }
            }
            pub fn insert(tree: &mut Ptr<Treap<T>>, tree_key: T, mut rng: &mut ThreadRng) -> Ptr<Treap<T>> {
                let mut tree = tree.take();
                let (mut tsm, mut tlg) = Treap::split(&mut tree, tree_key);
                let (mut sm, mut lg) = Treap::split_eq(&mut tlg, tree_key);
                if sm.is_some() {
                    get_mut!(sm).count += 1;
                    get_mut!(sm).upd_size();
                    Treap::merge(&mut tsm, &mut Treap::merge(&mut sm, &mut lg))
                } else {
                    Treap::merge(&mut tsm, &mut Treap::merge(&mut new_treap!(tree_key, rng), &mut lg))
                }
            }
            pub fn erase(tree: &mut Ptr<Treap<T>>, tree_key: T) -> Ptr<Treap<T>> {
                let mut tree = tree.take();
                let (mut tsm, mut tlg) = Treap::split(&mut tree, tree_key);
                let (mut sm, mut lg) = Treap::split_eq(&mut tlg, tree_key);
                if sm.is_some() && get!(sm).count > 1 {
                    get_mut!(sm).count -= 1;
                    get_mut!(sm).upd_size();
                    Treap::merge(&mut tsm, &mut Treap::merge(&mut sm, &mut lg))
                } else {
                    Treap::merge(&mut tsm, &mut lg)
                }
            }
            pub fn get_by_rank(tree: &Ptr<Treap<T>>, rank: usize) -> T {
                assert!(tree.is_some());
                if rank == 0 { return get!(tree).tree_key; }
                let l_size = Treap::get_size(&get!(tree).lc);
                let cur_count = get!(tree).count;
                if l_size + cur_count < rank {
                    Treap::get_by_rank(&get!(tree).rc, rank - l_size - cur_count)
                } else if l_size < rank {
                    get!(tree).tree_key
                } else {
                    Treap::get_by_rank(&get!(tree).lc, rank)
                }
            }
        }
    }
}
