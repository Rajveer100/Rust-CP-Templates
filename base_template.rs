use std::io;
use std::io::{BufWriter, stdout, Stdout, Write};

// use std::mem::{swap, replace};
// use std::cmp::{max, min, Ordering, Reverse};

// use std::collections::{
//     BTreeMap, BTreeSet, BinaryHeap,
//     HashMap, HashSet, VecDeque
// };

use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Rem, RemAssign,
    Neg
};

// use std::rc::Rc;
// use std::cell::RefCell;

// use itertools::Itertools;

// const MOD: i64 = 1_000_000_007;
// const MOD: i64 = 998_244_353;

// const BIG_INF: i64 = 1_000_000_000_000_000_000;
// const SMALL_INF: i32 = 1_000_000_000;

// const MAX_LIM: i32 = 2_000_000;

// const DIR4: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
// const DIR8: [(i32, i32); 8] = [
//     (1, 0), (0, 1), (-1, 0), (0, -1),
//     (1, 1), (1, -1), (-1, 1), (-1, -1)
// ];

fn solve(reader: &mut Reader, writer: &mut BufWriter<Stdout>) {

}

#[allow(dead_code)]
fn gcd<T>(a: T, b: T) -> T
    where T: Default + Copy + PartialEq + PartialOrd + Rem<Output = T> {
    return if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[allow(dead_code)]
fn lcm<T>(a: T, b: T) -> T
    where T: Default + Copy + PartialEq + PartialOrd + Rem<Output = T> +
    Mul<Output = T> + Div<Output = T> {
    (a * b) / gcd(a, b)
}

#[derive(Default)]
struct Reader {
    buffer: Vec<String>
}

#[allow(dead_code)]
impl Reader {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok()
                    .expect("Failed to parse!");
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read!");

            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    fn next_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }

    fn next_lines(&self) -> Vec<String> {
        let mut input: Vec<String> = Vec::new();
        for line in io::stdin().lines() {
            if line.as_ref().unwrap().len() == 0 {
                break
            }
            input.push(line.unwrap());
        }
        input
    }
}

fn main() {
    let mut reader = Reader::default();
    let writer = &mut BufWriter::new(stdout());

    let is_multi_test: bool = true;
    let mut t: i32 = 1;

    if is_multi_test {
        t = reader.next();
    }

    for _i in 0..t {
        // write!(writer, "Case #{}: ", _i + 1).ok();
        solve(&mut reader, writer);
    }
}