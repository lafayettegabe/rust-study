mod my_funcs;
mod other_funcs;

use crate::my_funcs::{add_five, add_ten};
use crate::other_funcs::minus_funcs::subtract_ten;

fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    x = x + y;
    println!("x is {}", x);

    x = add_ten(x);
    println!("x is {}", x);

    x = subtract_ten(x);
    println!("x is {}", x);
}
