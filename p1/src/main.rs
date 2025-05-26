/// Problem Statement:
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
/// Find the sum of all the multiples of 3 or 5 below 1000.

struct EulerP1InputArgs {
    max: u64,
    multiples: Vec<u64>
}

struct EulerP1Series {
    current: u64,
    args: EulerP1InputArgs
}

fn is_multiple(a: u64, bv: &Vec<u64>) -> bool {
    
    fn is_multiple(a: u64, b: u64) -> bool {
        a % b == 0
    }

    for b in bv {
        if is_multiple(a, *b) {
            return true;
        }
    }
    return false;
}

impl Iterator for EulerP1Series {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i: u64 = self.current+1;

        loop {
            if i >= self.args.max {
                return None
            }
            else if is_multiple(i, &self.args.multiples) {
                self.current = i;
                return Some(i)
            }
            i += 1;
        }
    }
}

impl EulerP1Series {
    fn sum(self) -> u64 {
        let mut sum: u64 = 0;
        for i in self {
            sum = sum + i;
        }
        return sum;
    }
}

fn main() {
    let args: EulerP1InputArgs = EulerP1InputArgs { 
        max: 1000,
        multiples: vec![3, 5] 
    };

    let series: EulerP1Series = EulerP1Series { 
        current: 1, 
        args: args 
    };

    println!("{}", series.sum())

}
