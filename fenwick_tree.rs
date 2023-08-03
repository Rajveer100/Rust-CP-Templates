#[derive(Debug, Clone, PartialEq)]
struct FenwickTree<T> where
    T: Default + Copy + PartialEq +
    Add + Sub + AddAssign + SubAssign {
    tree: Vec<T>
}

#[allow(dead_code)]
impl<T> FenwickTree<T> where
    T: Default + Copy + PartialEq +
    Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign {
    fn new(max_size: usize) -> Self {
        Self { tree: vec![T::default(); max_size + 1] }
    }

    fn from(a: &Vec<T>) -> Self {
        let mut tree = Self { tree: vec![T::default(); a.len() + 1] };
        for (ind, &x) in a.iter().enumerate() {
            tree.inc(ind as i32, x);
        }
        tree
    }

    fn inc(&mut self, mut ind: i32, val: T) {
        ind += 1;
        while ind < self.tree.len() as i32 {
            self.tree[ind as usize] += val;
            ind += ind & (-ind);
        }
    }

    fn get_for(&mut self, l: i32, r: i32) -> T {
        if l > r {
            panic!("Invalid Range! (l <= r)");
        }
        self.get(r) - self.get(l - 1)
    }

    fn get(&mut self, mut ind: i32) -> T {
        ind += 1;
        let mut sum: T = T::default();
        while ind > 0 {
            sum += self.tree[ind as usize];
            ind -= ind & (-ind);
        }
        sum
    }
}