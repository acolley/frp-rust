#![feature(globs)]

extern crate rustfrp;

use rustfrp::signal::*;

fn main() {
    let (input, mut four)   = constant(4i32);
    let timestwo            = lift(&four, |x: i32| { println!("timestwo"); x * 2 });
    let times               = lift2(&mut four, &timestwo, |x: i32, y| { println!("times"); x * y });
    let _output             = lift(&times, |x: i32| println!("{}", x));
    input.send(());
}
