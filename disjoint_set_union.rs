#[derive(Debug, Clone, PartialEq)]
struct Dsu {
    par: Vec<usize>,
    size: Vec<usize>,
}

#[allow(dead_code)]
impl Dsu {
    fn new(max_size: usize) -> Self {
        let mut init_dsu = Self {
            par: vec![0; max_size],
            size: vec![0; max_size],
        };
        for i in 0..max_size {
            init_dsu.par[i] = i;
            init_dsu.size[i] = 1;
        }
        init_dsu
    }

    fn find(&self, v: usize) -> usize {
        if v == self.par[v] {
            return v;
        }
        self.find(self.par[v])
    }

    fn union(&mut self, mut u: usize, mut v: usize) {
        u = self.find(u);
        v = self.find(v);
        if u != v {
            if self.size[u] < self.size[v] {
                swap(&mut u, &mut v);
            }
            self.par[v] = u;
            self.size[u] += self.size[v];
        }
    }
}