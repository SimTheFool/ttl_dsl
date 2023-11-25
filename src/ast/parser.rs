use pest::Span;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ast/ttl.pest"]
pub struct TTLParser;

pub fn span_into_string(span: Span) -> String {
    span.as_str().to_string()
}
pub fn span_into_str(span: Span) -> &str {
    span.as_str()
}
