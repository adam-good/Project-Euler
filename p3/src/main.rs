// use lib::prime_sieve::prime_sieve;
// use lib::new_bin_tree::BinaryTree;


// struct FactorTree {
//     N: u64,
//     tree: BinaryTree<u64>,
// }

// impl FactorTree {
//     fn new(n: u64) -> Self {
//         let mut tree: BinaryTree<u64> = BinaryTree::new();
//         tree.insert_center(n);

//         FactorTree{
//             N: n,
//             tree: tree,
//         }
//     }

//     // This probably doesn't need to be imperative
//     fn factor(&mut self) {
//         let mut n = match self.tree.value() {
//             None => panic!("This should never be reached."),
//             Some(n) => n
//         };
//         let mut primes = prime_sieve(n);

//         let mut p = match primes.next() {
//             None => panic!("You somehow ran out of primes before starting??"),
//             Some(x) => x
//         };
        
//         loop {
//             if n % p == 0 {
//                 n = n / p;
//                 self.tree.insert_left(p);
//                 self.tree.insert_right(n);
                
//             } else {
//                 p = match primes.next() {
//                     None => panic!("You somehow ran out of primes..."),
//                     Some(x) => x
//                 }
//             }

//             if n == 1 {
//                 // self.tree.insert_left(n); // Do I care?
//                 break;
//             }
//         };
//     }
// }

fn main() {
    // let n = 13195;
    // let mut ft = FactorTree::new(n);
    // ft.factor();

    // // let factors = ft.tree.collect();
    // // for f in factors {
    // //     print!("{} ", f);
    // // }
    // // println!();

    // let primes = ft.tree.leaves();
    // for f in primes {
    //     print!("{} ", f)
    // }
    // println!()
}
