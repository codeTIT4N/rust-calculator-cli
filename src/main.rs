use std::env::{args, Args};

fn main() {
    let mut args: Args = args(); //note: here we are declring args as a mutable variable

    let first = args.nth(1).unwrap(); //this will give the first variables value
    println!("{:?}", first);
}
