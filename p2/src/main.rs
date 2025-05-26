
struct FiniteFibonacci {
    curr: u64,
    next: u64,
    max: u64
}

impl Iterator for FiniteFibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        if current >= self.max {
            return None
        }

        self.curr = self.next;
        self.next = current + self.next;

        return Some(current);
    }
}

fn fibonacci() -> FiniteFibonacci {
    FiniteFibonacci { curr: 1, next: 2, max: 4_000_000 }
}

fn main() {
    let s: u64 = fibonacci()
                .filter(|x| x % 2 == 0)
                .sum();

    println!("{}", s)
}
