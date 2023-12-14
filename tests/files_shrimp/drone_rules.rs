pub const DRONE_RULES: &str = r#"

{
    price: 0
}

@TRANSFORM FINAL_STATS
> $.hit += floor($.resistance /2)
"#;
