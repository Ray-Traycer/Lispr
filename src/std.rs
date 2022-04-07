use crate::vm::SceneVM;
use crate::interpreter::types::{Type, StringType};

macro_rules! call {
    ( $($f:ident),* ) => {
        pub fn callFunc(func_name: &str, expr_vec: &[Type], vm: *mut SceneVM) -> Type
        {
            match func_name{
                $(
                    stringify!($f) => $f(expr_vec, vm),
                )*
                _ => {
                    let mutvm = unsafe{&mut (*vm)};
                    if let Some(func) = mutvm.get_function(&func_name.to_lowercase()){
                        if let Some(result) = func(expr_vec, vm){
                            return result;
                        }
                    }
                    StringType("None".to_owned()).into()
                }
            }

        }
    }
}

pub fn print(expr_vec: &[Type], vm: *mut SceneVM) -> Type{
    let print_text = expr_vec.iter().map(|argument| argument.string(vm)).collect::<Vec<String>>().join(" ");
    println!("{}", print_text);
    StringType("None".to_owned()).into()
}

pub fn Loop(expr_vec: &[Type], vm: *mut SceneVM) -> Type{
    let amount = expr_vec[0].string(vm).parse::<usize>().unwrap();
    let mut last: Type = StringType("None".to_owned()).into();
    for _ in 0..amount{
        last = expr_vec[1].eval(vm);
    }
    last
}


call!(print, Loop);
