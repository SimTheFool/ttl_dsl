pub const DRONE_RULES: &str = r#"

{
    price: 0
}

@TRANSFORM FINAL_STATS
> $.stats.hit += floor($.stats.resistance /2)
"#;
