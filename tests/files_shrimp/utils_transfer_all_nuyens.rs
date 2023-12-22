pub const UTILS_TRANSFER_ALL_NUYENS: &str = r#"
{
    nuyens: 0
}

@TRANSFORM BANK
> $.nuyens += nuyens
> nuyens = 0
"#;
