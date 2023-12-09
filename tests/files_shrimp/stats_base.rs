pub const STATS_BASE: &str = r#"
{
    con: con
    con_mod: 0
    agi: agi
    agi_mod: 0
    rea: rea
    rea_mod: 0
    for: for
    for_mod: 0
    vol: vol
    vol_mod: 0
    log: log
    log_mod: 0
    int: int
    int_mod: 0
    cha: cha
    cha_mod: 0

    ess: ess
    edge: 4

    ["con"]
    resist_phy: 0
    ["vol"]
    resist_ment: 0
    ["rea" "int"]
    def_phy: 0
    ["vol" "int"]
    def_ment: 0

    init_dice: 1
    ["rea" "int"]
    init_base: 0

    action_maj: 1
    action_min: 1

    hit_phy: 8
    hit_stun: 8
    hit_over: 0

    ["con" "vol"]
    heal: 0
}

@TRANSFORM FINAL_STATS
> $.resist_phy = $.con
> $.resist_ment = $.vol
> $.def_phy = $.rea + $.int
> $.def_ment = $.vol + $.int
> $.action_min = $.action_min + $.init_dice
> $.hit_phy = $.hit_phy + floor($.con / 2)
> $.hit_stun = $.hit_stun + floor($.vol / 2)
> $.hit_over = $.hit_over + $.con - $.con_mod
> $.heal = $.heal + $.con + $.vol
> $.init_base = $.init_base + $.rea + $.int
"#;
