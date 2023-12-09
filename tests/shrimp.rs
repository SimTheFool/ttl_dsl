use custom_dsl::domain::resolution::ResolvedResourceValue;
use files_shrimp::*;
use utils::MockedApp;

#[macro_use]
mod utils;
mod files_shrimp;

const INDEX: &str = r#"
    {
        stats:
        {
            << ./stats_base
                with con : 1
                with agi : 4
                with rea : 2
                with for : 1
                with vol : 4
                with log : 6
                with int : 4
                with cha : 2
                with ess : 6
            << ./stats_techno
                with res : 7
                with submersion : 1
        }
    }
"#;

#[test]
fn it_shoud_assemble_shrimp() {
    let (app, resolver, config) = MockedApp::new();

    config.borrow_mut().add_layer("FINAL_STATS");
    config.borrow_mut().add_layer("FINAL_STATS_END");
    resolver.borrow_mut().mock_file("./stats_base", STATS_BASE);
    resolver
        .borrow_mut()
        .mock_file("./stats_techno", STATS_TECHNO);

    let resolved_resources = app.assemble_from_str(INDEX);
    let resolved_resources = unwrap_or_print_error!(resolved_resources);

    assert_resource_at!(resolved_resources : "stats.con" => Number 1.0);
    assert_resource_at!(resolved_resources : "stats.con_mod" => Number 0.0);
    assert_resource_at!(resolved_resources : "stats.agi" => Number 4.0);
    assert_resource_at!(resolved_resources : "stats.agi_mod" => Number 0.0);
    assert_resource_at!(resolved_resources : "stats.rea" => Number 2.0);
    assert_resource_at!(resolved_resources : "stats.rea_mod" => Number 0.0);
    assert_resource_at!(resolved_resources : "stats.for" => Number 1.0);
    assert_resource_at!(resolved_resources : "stats.for_mod" => Number 0.0);
    assert_resource_at!(resolved_resources : "stats.vol" => Number 4.0);
    assert_resource_at!(resolved_resources : "stats.vol_mod" => Number 0.0);
    assert_resource_at!(resolved_resources : "stats.log" => Number 6.0);
    assert_resource_at!(resolved_resources : "stats.log_mod" => Number 0.0);
    assert_resource_at!(resolved_resources : "stats.int" => Number 4.0);
    assert_resource_at!(resolved_resources : "stats.int_mod" => Number 0.0);
    assert_resource_at!(resolved_resources : "stats.cha" => Number 2.0);
    assert_resource_at!(resolved_resources : "stats.cha_mod" => Number 0.0);
    assert_resource_at!(resolved_resources : "stats.ess" => Number 6.0);
    assert_resource_at!(resolved_resources : "stats.edge" => Number 4.0);
    assert_resource_at!(resolved_resources : "stats.resist_phy" => Number 1.0);
    assert_resource_at!(resolved_resources : "stats.resist_ment" => Number 4.0);
    assert_resource_at!(resolved_resources : "stats.def_phy" => Number 6.0);
    assert_resource_at!(resolved_resources : "stats.def_ment" => Number 8.0);
    assert_resource_at!(resolved_resources : "stats.init_dice" => Number 1.0);
    assert_resource_at!(resolved_resources : "stats.init_base" => Number 6.0);
    assert_resource_at!(resolved_resources : "stats.action_maj" => Number 1.0);
    assert_resource_at!(resolved_resources : "stats.action_min" => Number 2.0);
    assert_resource_at!(resolved_resources : "stats.hit_phy" => Number 8.0);
    assert_resource_at!(resolved_resources : "stats.hit_stun" => Number 10.0);
    assert_resource_at!(resolved_resources : "stats.hit_over" => Number 1.0);
    assert_resource_at!(resolved_resources : "stats.heal" => Number 5.0);

    assert_resource_at!(resolved_resources : "stats.res" => Number 7.0);
    assert_resource_at!(resolved_resources : "stats.submersion" => Number 1.0);
    assert_resource_at!(resolved_resources : "stats.resist_drain" => Number 3.0);
    assert_resource_at!(resolved_resources : "stats.firewall" => Number 4.0);
    assert_resource_at!(resolved_resources : "stats.traitement" => Number 6.0);
    assert_resource_at!(resolved_resources : "stats.corruption" => Number 4.0);
    assert_resource_at!(resolved_resources : "stats.attaque" => Number 2.0);
}
