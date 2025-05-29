

// NOTE: I wrote this because I misread a problem.
//       it's crap but might be useful later
struct PrimeSieve {
    num: u64,
    curr: u64,
    list: Vec<u64>
}

impl Iterator for PrimeSieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.num {
            return None
        }
        let current_prime = self.curr;

        // TODO: I am sure this is a mess. I need to review borrowing
        //       and iterators
        self.list = self.list.iter()
                    .filter(|x| *x % current_prime != 0)
                    .map(|x| *x)
                    .collect();

        self.curr = match self.list.first() {
            Some(x) => *x,
            None => u64::MAX
        };

        return Some(current_prime);        
    }
}

fn prime_sieve(n: u64) -> PrimeSieve {
    PrimeSieve { num: n, curr: 2, list: (2..n).collect() }
}

mod tests {
    use super::*;

    #[test]
    fn prime_sieve_test() {
        let num: u64 = 25;
        let target: Vec<u64> = vec![2,3,5,7,11,13,17,19,23]; // primes < 25
        let sieve = prime_sieve(num);

        let result: Vec<u64> = sieve.collect();

        assert_eq!(target, result);

    }
}