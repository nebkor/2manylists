extern crate lists;

use std::collections::VecDeque;
use lists::second::List;


fn main() {
    {
        // Create a Vector of values.
        let input = vec![1, 2, 3];
        let iterator = input.iter();
        let mapped = iterator.map(|&x| {
            return x * 2;
        });
        // Gather the result in a RingBuf.
        let output = mapped.collect::<VecDeque<_>>();
        println!("{:?}", output);
    }

    {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let iter = list.iter();
        let mapped = iter.map(|&x| {
            return x;
        });

        let output = mapped.collect::<Vec<_>>();
        println!("vector from list: {:?}", output);
    }
}
// Outputs [2, 4, 6]
