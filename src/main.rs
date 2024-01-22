#![allow(dead_code, unused_variables)]
use std::{future::Future, fs::read_to_string};

fn main() {
    println!("Hello, world!");
    let x = foo2();
}


async fn foo1() -> usize {
    println!("foo");
    0
}
fn foo2() -> impl Future<Output = usize> {
    async {
        // First time
        println!("foo1");
        read_to_string("file1").await; // wait here
        println!("foo1");
        read_to_string("file2").await; // wait here
        println!("foo1");
        read_to_string("file3").await; // wait here
        println!("foo1");
        read_to_string("file4").await; // wait here
        println!("foo2");
        0
    }
}
