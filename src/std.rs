use crate::interpreter::Type;
macro_rules! call {
    ( $($f:ident),* ) => {
        pub fn callFunc(func_name: &str, expr_vec: &mut [Type]) -> Type
        {
            match func_name{
                $(
                    stringify!($f) => $f(expr_vec),
                )*
                _ => Type::String("None".to_string())
            }

        }
    }
}

pub fn print(expr_vec: &mut [Type]) -> Type{
    let print_text = expr_vec.iter_mut().map(|argument| argument.string()).collect::<Vec<String>>().join(" ");
    println!("{}", print_text);
    Type::String("None".to_string())
}

pub fn Loop(expr_vec: &mut [Type]) -> Type{
    let amount = expr_vec[0].string().parse::<usize>().unwrap();
    let mut last = Type::String("None".to_string());
    for _ in 0..amount{
        last = expr_vec[1].eval();
    }
    last
}

call!(print, Loop);
