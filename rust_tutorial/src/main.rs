#![allow(unused)]

use std::{io, string};
use std::io::{Write,BufRead,BufReader,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("What is your name?");
    let mut name:string = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(buf:&mut name).expect("didn't receive input");
    println!("Hello, world!");
}
