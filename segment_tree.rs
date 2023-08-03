#[derive(Debug, Clone, PartialEq)]
struct SegmentTree<T> where
    T: Default + Copy + PartialEq +
    Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign +
    Mul<Output = T> + MulAssign {
    tree: Vec<T>,
    lazy_tree: Vec<T>
}

#[allow(dead_code)]
impl<T> SegmentTree<T> where
    T: Default + Copy + PartialEq +
    Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign +
    Mul<Output = T> + MulAssign + From<i32> {
    fn new(a: &Vec<T>) -> Self {
        let n: usize = Self::get_closest_pow2(a.len());
        let mut tree = Self {
            tree: vec![T::default(); 2 * n],
            lazy_tree: vec![T::default(); 2 * n],
        };
        Self::build(1, 0, n - 1, &mut tree.tree, &a);
        tree
    }

    fn get(&mut self, p: usize) -> T {
        self.get_for(p, p)
    }

    fn get_for(&mut self, l: usize, r: usize) -> T {
        if l > r {
            panic!("Invalid Range! (l <= r)");
        }
        self.get_val(1, 0, self.tree.len() / 2 - 1, l, r)
    }

    fn inc(&mut self, ind: usize, val: T) {
        self.inc_for(ind, ind, val);
    }

    fn inc_for(&mut self, l: usize, r: usize, val: T) {
        if l > r {
            panic!("Invalid Range! (l <= r)");
        }
        self.inc_seg(1, 0, self.tree.len() / 2 - 1, l, r, val);
    }

    fn inc_seg(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize, val: T) {
        self.update_lazy(node, left, right);

        if left > r || right < l {
            return;
        }

        if left >= l && right <= r {
            self.tree[node] += val * ((right - left + 1) as i32).into();
            if left != right {
                self.lazy_tree[2 * node] += val;
                self.lazy_tree[2 * node + 1] += val;
            }
        } else {
            let mid: usize = (left + right) >> 1;
            self.inc_seg(2 * node, left, mid, l, r, val);
            self.inc_seg(2 * node + 1, mid + 1, right, l, r, val);
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }

    fn get_val(&mut self, node: usize, left: usize, right: usize, l: usize, r: usize) -> T {
        self.update_lazy(node, left, right);

        if left > r || right < l {
            return T::default();
        }

        if left >= l && right <= r {
            return self.tree[node];
        }

        let mid: usize = (left + right) >> 1;
        let left_val: T = self.get_val(2 * node, left, mid, l, r);
        let right_val: T = self.get_val(2 * node + 1, mid + 1, right, l, r);
        left_val + right_val
    }

    fn update_lazy(&mut self, node: usize, left: usize, right: usize) {
        if self.lazy_tree[node] != T::default() {
            let lazy_val: T = self.lazy_tree[node];
            self.tree[node] += lazy_val * ((right - left + 1) as i32).into();
            if left != right {
                self.lazy_tree[2 * node] += lazy_val;
                self.lazy_tree[2 * node + 1] += lazy_val;
            }
            self.lazy_tree[node] = T::default();
        }
    }

    fn build(node: usize, left: usize, right: usize, tree: &mut Vec<T>, a: &Vec<T>) {
        if left == right {
            if left <= a.len() - 1 {
                tree[node] = a[left];
            }
        } else {
            let mid: usize = (left + right) >> 1;
            Self::build(2 * node, left, mid, tree, a);
            Self::build(2 * node + 1, mid + 1, right, tree, a);
            tree[node] = tree[2 * node] + tree[2 * node + 1];
        }
    }

    fn get_closest_pow2(val: usize) -> usize {
        if val & (val - 1) == 0 {
            return val;
        }
        (usize::MAX >> val.leading_zeros()) + 1
    }
}