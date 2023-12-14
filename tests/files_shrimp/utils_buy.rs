pub const UTILS_BUY: &str = r#"

{
    price: 0
}

@TRANSFORM BUY_FINAL
> $.price += $cost
"#;
