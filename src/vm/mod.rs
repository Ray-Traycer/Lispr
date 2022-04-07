use std::collections::HashMap;

use crate::interpreter::{types::Type, interpreter::ParseFile};

type Func = Box<dyn FnMut(&[Type], *mut SceneVM) -> Option<Type>>;

pub struct SceneVM<'a>{
    pub std: HashMap<String, Box<dyn 'a + FnMut(&[Type], *mut SceneVM) -> Option<Type>>>,
}

impl<'a> SceneVM<'a>{
    pub fn new() -> Self{
        SceneVM{
            std: HashMap::new(),
        }
    }
    pub fn run(&mut self, file: &str){
        let ptr = &mut *self as _;
        println!("running {}", file);
        let now = std::time::Instant::now();
        let code = ParseFile(file).unwrap();
        for statement in code{
            statement.eval(ptr);
        }
        println!("done. took {}s", now.elapsed().as_secs());
    }

    pub fn get_function(&mut self, name: &str) -> Option<&mut Box<dyn 'a + FnMut(&[Type], *mut SceneVM) -> Option<Type>>>{
        let a = self.std.get_mut(name);
        a
    }
    pub fn add_std<CB: 'a + FnMut(&[Type], *mut SceneVM) -> Option<Type>>(&mut self, name: &str, func: CB){
        self.std.insert(name.to_string(), Box::new(func));
    }
}