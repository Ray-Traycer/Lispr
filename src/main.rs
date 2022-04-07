#![allow(non_snake_case)]

use ::std::cell::Cell;

use parse::vm::SceneVM;

pub mod std;
pub mod interpreter;
pub mod vm;


fn main(){
    let i = Cell::new(0.0);
    let mut vm = SceneVM::new();
    vm.add_std("i", |_, _|{
        i.set(i.get() + 1.0);
        None
    });
    vm.add_std("t", |_, _|{
        Some(i.get().into())
    });
    vm.run("test.ss");
    
}

