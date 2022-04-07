use ::std::fs::read_to_string;
use crate::interpreter::types::*;

peg::parser!{
    grammar VoxelParser() for str {
        rule _ = [' '| '\t' | '\n' | '\r' |'\u{A}']*
        rule number() -> Type
            = _ n:$(['0'..='9' | '.' | '-']+) _ { Box::new(Number(n.parse::<f32>().unwrap())) }
        rule symbol() -> Type
        = _ n:$(['A'..='z' | '*']+) _ { Box::new(Atom(n.to_string())) }
        rule string() -> Type
            = _ "\"" n:$(['A'..='z' | ' ']+) "\"" _ { StringType(n.to_string()).into() }
    
        pub(crate) rule parse() -> Type = precedence!{
            _ "(" _ expr:(parse()*) _ ")" _ {Expression(expr).into()}
            --
            n:number() {n}
            n:symbol() {n}
            n:string() {n}
        }
    
        pub(crate) rule ParseFile() -> Vec<Type> = precedence!{
            code:((parse())* ) {
                code
            }
        }
    }
}

pub fn ParseFile(file: &str) -> Result<Vec<Type>, String>{
    let parser = VoxelParser::ParseFile(&read_to_string(file).unwrap());
    match parser{
        Ok(code) => Ok(code),
        Err(e) => Err(format!("{}", e)),
    }
}