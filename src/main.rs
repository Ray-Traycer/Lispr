#![allow(non_snake_case)]

use crate::interpreter::ParseFile;

pub mod std;
pub mod interpreter;


fn main(){
    let mut parser = ParseFile("test").unwrap();
    parser.eval();
}

