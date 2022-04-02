
use ::std::fs::read_to_string;

#[derive(Debug)]
pub enum Type{
    Expression(Vec<Type>),
    String(String),
    Atom(String),
    Number(f64),
}

impl Type{
    pub fn string(&mut self) -> String{
        match self{
            Type::String(s) => s.to_string(),
            Type::Atom(s) => s.to_string(),
            Type::Number(n) => n.to_string(),
            expr @ Type::Expression(_) => expr.eval().string(),
        }
    }
    pub fn eval(&mut self) -> Self{
        match self{
            Type::Expression(vec) => {
                if vec.len() == 0 {return Type::String("None".to_string())}
                let function_name = vec[0].string();
                let a = &mut vec[1..];
                crate::std::callFunc(&function_name,a)
            },
            Type::String(s)     => Type::String(s.to_string()),
            Type::Atom(s)       =>  Type::Atom(s.to_string()),
            Type::Number(n)       =>  Type::Number(*n),
        }
    }
}

peg::parser!{
    grammar VoxelParser() for str {
        rule _ = [' '| '\t' | '\n' | '\r' |'\u{A}']*
        rule number() -> Type
            = _ n:$(['0'..='9' | '.']+) _ { Type::Number(n.parse().unwrap()) }
        rule symbol() -> Type
        = _ n:$(['A'..='z' | '*']+) _ { Type::Atom(n.to_string()) }
        rule string() -> Type
            = _ "\"" n:$(['A'..='z' | ' ']+) "\"" _ { Type::String(n.to_string()) }
    
        pub(crate) rule parse() -> Type = precedence!{
            _ "(" _ expr:(parse()*) _ ")" _ {Type::Expression((expr))}
            --
            n:number() {n}
            n:symbol() {n}
            n:string() {n}
            
        }
    
        pub(crate) rule ParseFile() -> Type = precedence!{
            code:(parse()) {
                code
            }
        }
    }
}

pub fn ParseFile(file: &str) -> Result<Type, String>{
    let parser = VoxelParser::ParseFile(&read_to_string(file).unwrap());
    match parser{
        Ok(code) => Ok(code),
        Err(e) => Err(format!("{}", e)),
    }
}