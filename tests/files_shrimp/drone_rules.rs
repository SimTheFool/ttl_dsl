pub const DRONE_RULES: &str = r#"
@TRANSFORM FINAL_STATS
> $.stats.hit += floor($.stats.resistance /2)
"#;
