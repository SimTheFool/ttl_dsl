use super::parser::{span_into_str, span_into_string, Rule};
use pest_ast::FromPest;

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::identifier))]
pub struct Variable(#[pest_ast(outer(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::reference))]
pub struct Ref(#[pest_ast(outer(with(span_into_string)))] pub String);
impl Ref {
    pub fn get_var_name<'a>(&'a self) -> &'a str {
        self.0
            .strip_prefix("$")
            .expect("Variable does not start with $")
    }
}

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::string))]
pub struct StringLit(#[pest_ast(inner(with(span_into_string)))] pub String);

#[derive(Debug, PartialEq, FromPest)]
#[pest_ast(rule(Rule::number))]
pub struct Number(
    #[pest_ast(outer(with(span_into_str), with(str::parse::<f64>), with(Result::unwrap)))] pub f64,
);

#[cfg(test)]
mod tests {
    use crate::domain::ast::parser::TTLParser;
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn it_should_parse_identifier() {
        let str = "var01";
        let mut pairs = TTLParser::parse(super::Rule::identifier, str).unwrap();
        let variable = super::Variable::from_pest(&mut pairs).unwrap();

        assert_eq!(variable.0, "var01");
    }
}
