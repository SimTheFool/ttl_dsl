pub const STATS_TECHNO: &str = r#"
{
    res: res
    submersion: submersion
    ["vol"]
    resist_drain: 0

    firewall: 0
    traitement: 0
    corruption: 0
    attaque: 0
}

@TRANSFORM FINAL_STATS
> $.resist_drain = floor($.log / 2)
> $.firewall = $.vol
> $.traitement = $.log
> $.corruption = $.int
> $.attaque = $.cha
"#;
