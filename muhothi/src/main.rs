mod lib;
// use lib::{execute_bf};

fn main() {
    //break文が使える。
    let mut x = 7;
    loop {
        x -= 1;
        println!("{}", x);
        if x == 3 { break; }
    }
   
    //continue文が使える。
    for y in 0..10 {
        if y % 2 == 0 { continue;/* 飛ばすという意味 */ }
        
        println!("{}", y);
    }
}