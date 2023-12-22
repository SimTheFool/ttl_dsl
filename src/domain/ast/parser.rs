use pest::Span;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "domain/ast/ttl.pest"]
pub struct TTLParser;

pub fn span_into_string(span: Span) -> String {
    span.as_str().trim().to_string()
}
pub fn span_into_string_option(span: Span) -> Option<String> {
    let str = span.as_str().trim();
    if str.is_empty() {
        None
    } else {
        Some(str.to_string())
    }
}
pub fn span_into_str(span: Span) -> &str {
    span.as_str().trim()
}
