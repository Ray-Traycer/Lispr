
pub trait TypeTrait{
    fn eval(&self, vm: *mut SceneVM) -> Type;
    fn string(&self, _vm: *mut SceneVM) -> String{
        panic!("Not implemented")
    }
    fn as_any(&self) -> Option<&dyn Any>{
        None
    }
}

pub type Type = Box<dyn TypeTrait>;
pub struct Atom(pub String);
pub struct Expression(pub Vec<Type>);
pub struct Number(pub f32);
pub struct StringType(pub std::string::String);

pub struct Statements(pub Vec<Type>);

/* TYPETRAIT impls */

impl TypeTrait for StringType{
    fn eval(&self, _: *mut SceneVM) -> Type{
        Box::new(StringType(self.0.to_owned()))
    }
    fn string(&self, _: *mut SceneVM) -> String{
        self.0.clone()
    }
}

impl TypeTrait for Number{
    fn eval(&self, _: *mut SceneVM) -> Type{
        Number(self.0).into()
    }
    fn string(&self, _: *mut SceneVM) -> String{
        format!("{}", self.0)
    }
}

impl TypeTrait for Atom{
    fn eval(&self, _: *mut SceneVM) -> Type{
        Box::new(Atom(self.0.clone()))
    }
    fn string(&self, _: *mut SceneVM) -> String{
        self.0.clone()
    }
}

impl TypeTrait for Expression{
    fn eval(&self, vm: *mut SceneVM) -> Type {
        let vec = &self.0;
        if vec.len() == 0 {return "None".into()}
        let function_name = vec[0].string(vm);
        let a = &vec[1..];
        crate::std::callFunc(&function_name,a, vm)
    }

    fn string(&self, vm : *mut SceneVM) -> String {
        self.eval(vm).string(vm)
    }
}

impl TypeTrait for Statements{
    fn eval(&self, vm: *mut SceneVM) -> Type{
        let vec = &self.0;
        if vec.len() == 0 {return "None".into()}
        let mut last: Type = "None".into();
        for statement in vec{
            last = statement.eval(vm);
        }
        last
    }
    fn string(&self, vm: *mut SceneVM) -> String{
        self.eval(vm).string(vm)
    }
}

/* INTO traits */

impl Into<Type> for StringType{
    fn into(self) -> Type {
        Box::new(self)
    }
}

impl Into<Atom> for String{
    fn into(self) -> Atom {
        Atom(self)
    }
}

impl Into<Type> for Atom{
    fn into(self) -> Type {
        Box::new(self)
    }
}

impl Into<Type> for Expression{
    fn into(self) -> Type {
        Box::new(self)
    }
}


impl Into<Type> for &str{
    fn into(self) -> Type {
        Box::new(StringType(self.to_owned()))
    }
}


impl Into<Type> for Number{
    fn into(self) -> Type {
        Box::new(self)
    }
}

impl Into<Type> for f32{
    fn into(self) -> Type {
        Box::new(Number(self))
    }
}

use std::any::Any;

use crate::vm::SceneVM;