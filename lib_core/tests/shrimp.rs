use crate::utils::*;
use lib_core::domain::resolution::ResolvedResourceValue;
use regex::Regex;

#[macro_use]
mod utils;

#[test]
fn it_shoud_assemble_shrimp() {
    let (app, resolver, config) = MockedApp::new();
    macro_rules! mock_file {
        ($path:expr) => {{
            let stripped_path = $path.replace("/filesys", "");
            resolver
                .borrow_mut()
                .mock_file(&stripped_path, include_str!($path))
        }};
    }

    config.borrow_mut().add_layer("MODS");
    config.borrow_mut().add_layer("FINAL_STATS");
    config.borrow_mut().add_layer("FINAL_STATS_END");
    config.borrow_mut().add_layer("BUY_FINAL");
    config.borrow_mut().add_layer("BANK");

    mock_file!("./filesys/stats/base");
    mock_file!("./filesys/stats/techno");
    mock_file!("./filesys/metatypes/human");

    mock_file!("./filesys/traits/ami_des_sprites");
    mock_file!("./filesys/traits/bricoleur_prevoyant");
    mock_file!("./filesys/traits/paralysie_du_combat");
    mock_file!("./filesys/traits/rhinite_chronique");

    mock_file!("./filesys/skills/base");
    mock_file!("./filesys/skills/spec");
    mock_file!("./filesys/skills/mast");

    mock_file!("./filesys/identity/native");
    mock_file!("./filesys/identity/fake");
    mock_file!("./filesys/identity/contact");
    mock_file!("./filesys/lifestyles/squat");
    mock_file!("./filesys/identity/licence");

    mock_file!("./filesys/actions/recharger");

    mock_file!("./filesys/objects/drones/base");
    mock_file!("./filesys/objects/drones/crawler");
    mock_file!("./filesys/objects/drones/kanmushi");
    mock_file!("./filesys/objects/mods/monture");

    mock_file!("./filesys/objects/accessories/guncam");
    mock_file!("./filesys/objects/mods/chasse_big");
    mock_file!("./filesys/objects/mods/canon_long");
    mock_file!("./filesys/objects/mods/disassembly_kit");

    mock_file!("./filesys/objects/guns/crockett");
    mock_file!("./filesys/objects/guns/actions/shot");
    mock_file!("./filesys/objects/guns/actions/shot_semi");
    mock_file!("./filesys/objects/guns/actions/shot_rafale");

    mock_file!("./filesys/utils/quantity_buy");
    mock_file!("./filesys/utils/quality_buy");
    mock_file!("./filesys/utils/quantity");
    mock_file!("./filesys/utils/transfer_all_nuyens");

    let shrimp_index = include_str!("./filesys/shrimp");
    let resources = app.assemble_from_str(shrimp_index);
    let resources = print_unwrap!(resources);

    /* Testing base stats */
    assert_resource_at!(resources : "stats.con" => Number 1.0);
    assert_resource_at!(resources : "stats.con_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.agi" => Number 4.0);
    assert_resource_at!(resources : "stats.agi_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.rea" => Number 2.0);
    assert_resource_at!(resources : "stats.rea_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.for" => Number 1.0);
    assert_resource_at!(resources : "stats.for_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.vol" => Number 4.0);
    assert_resource_at!(resources : "stats.vol_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.log" => Number 6.0);
    assert_resource_at!(resources : "stats.log_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.int" => Number 4.0);
    assert_resource_at!(resources : "stats.int_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.cha" => Number 2.0);
    assert_resource_at!(resources : "stats.cha_mod" => Number 0.0);
    assert_resource_at!(resources : "stats.ess" => Number 6.0);
    assert_resource_at!(resources : "stats.edge" => Number 4.0);
    assert_resource_at!(resources : "stats.resist_phy" => Number 1.0);
    assert_resource_at!(resources : "stats.resist_ment" => Number 4.0);
    assert_resource_at!(resources : "stats.def_phy" => Number 6.0);
    assert_resource_at!(resources : "stats.def_ment" => Number 8.0);
    assert_resource_at!(resources : "stats.init_dice" => Number 1.0);
    assert_resource_at!(resources : "stats.init_base" => Number 6.0);
    assert_resource_at!(resources : "stats.action_maj" => Number 1.0);
    assert_resource_at!(resources : "stats.action_min" => Number 2.0);
    assert_resource_at!(resources : "stats.hit_phy" => Number 8.0);
    assert_resource_at!(resources : "stats.hit_stun" => Number 10.0);
    assert_resource_at!(resources : "stats.hit_over" => Number 1.0);
    assert_resource_at!(resources : "stats.heal" => Number 5.0);
    assert_resource_at!(resources : "stats.res" => Number 7.0);
    assert_resource_at!(resources : "stats.submersion" => Number 1.0);
    assert_resource_at!(resources : "stats.resist_drain" => Number 3.0);
    assert_resource_at!(resources : "stats.firewall" => Number 4.0);
    assert_resource_at!(resources : "stats.traitement" => Number 6.0);
    assert_resource_at!(resources : "stats.corruption" => Number 4.0);
    assert_resource_at!(resources : "stats.attaque" => Number 2.0);

    /* Testing skills */
    assert_resource_at!(resources : "skills.combat_rapproché.score" => Number 1.0);
    assert_resource_at!(resources : "skills.perception.score" => Number 1.0);
    assert_resource_at!(resources : "skills.furtivité.score" => Number 1.0);
    assert_resource_at!(resources : "skills.athlétisme.score" => Number 3.0);
    assert_resource_at!(resources : "skills.électronique.score" => Number 4.0);
    assert_resource_at!(resources : "skills.ingénierie.score" => Number 6.0);
    assert_resource_at!(resources : "skills.pilotage.score" => Number 6.0);
    assert_resource_at!(resources : "skills.technomancie.score" => Number 6.0);

    /* Testing identities */
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.contacts.[a-zA-Z0-9]+.name" => String "D-Boss");
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.contacts.[a-zA-Z0-9]+.loyalty" => Number 4.0);
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.contacts.[a-zA-Z0-9]+.connection" => Number 4.0);
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.contacts.[a-zA-Z0-9]+.description" => String "Decker fan de complot");

    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.contacts.[a-zA-Z0-9]+.name" => String "Terrance");
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.contacts.[a-zA-Z0-9]+.loyalty" => Number 3.0);
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.contacts.[a-zA-Z0-9]+.connection" => Number 2.0);
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.contacts.[a-zA-Z0-9]+.description" => String "Ouvrier de casse militaire d'ARES");

    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.name" => String "Laurence Guinvite");
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.quality" => Number 4.0);
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.lifestyles.[a-zA-Z0-9]+.name" => String "squatteur");

    /* Testing licences */
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.licences.[a-zA-Z0-9]+.name" => String "Concierge de chantier");
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.licences.[a-zA-Z0-9]+.description" => Null);
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.licences.[a-zA-Z0-9]+.quality" => Number 4.0);

    /* Testing traits */
    assert_resource_at!(resources : "traits.ami_des_sprites.description" => String "__A1__ lorsque vous compilez ou inscrivez un sprite machine.");
    assert_resource_at!(resources : "traits.bricoleur_prévoyant.description" => String "__A1__ lorsque vous utilisez une machine que vous avez bricolé.");
    assert_resource_at!(resources : "traits.paralysie_du_combat.description" => String "Au premier round, vous ne pouvez pas vous déplacer et vous jouez en dernier.");
    assert_resource_at!(resources : "traits.rhinite_chronique.description" => String "Vous éternuez souvent. __D1__ lors des tests de discrétion.");

    /* Testing tags */
    assert_resource_at!(resources : "tags.[a-zA-Z0-9]+" => String "13 ans");
    assert_resource_at!(resources : "tags.[a-zA-Z0-9]+" => String "technorigger");
    assert_resource_at!(resources : "tags.[a-zA-Z0-9]+" => String "humain");

    /* Testing drones */
    assert_resource_at!(resources : "inventory.Crawler.stats.hit" => Number 11.0);
    assert_resource_number!(resources : r#"inventory.Crawler.slots.[a-zA-Z0-9]+.name"# => 2);

    /* Testing crockett */
    assert_resource_at!(resources : "inventory.Crockett.manufacturer" => String "Cavalier Arms");
    assert_resource_at!(resources : "inventory.Crockett.status" => String "illegal");

    assert_resource_at!(resources : "inventory.Crockett.actions.Tir.damage" => Number 5.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir.ammo" => Number 1.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir.ranges.contact" => Number -2.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir.ranges.near" => Number 0.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir.ranges.short" => Number 1.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir.ranges.mid" => Number 2.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir.ranges.far" => Number 1.0);

    assert_resource_at!(resources : "inventory.Crockett.actions.Tir_semi_auto.damage" => Number 6.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir_semi_auto.ammo" => Number 2.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir_semi_auto.ranges.contact" => Number -3.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir_semi_auto.ranges.near" => Number -1.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir_semi_auto.ranges.short" => Number 1.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir_semi_auto.ranges.mid" => Number 2.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.Tir_semi_auto.ranges.far" => Number 1.0);

    assert_resource_at!(resources : "inventory.Crockett.actions.recharger.ammo" => Number 250.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.guncam.major" => Number 0.0);
    assert_resource_at!(resources : "inventory.Crockett.actions.guncam.minor" => Number 1.0);

    /* Testing buy util */
    assert_resource_at!(resources : "inventory.Crockett.price" => Number 11350.0);
    assert_resource_at!(resources : "inventory.Crawler.price" => Number {9500.0 + 2500.0 + 2500.0});
    assert_resource_at!(resources : "inventory.Kanmushi.price" => Number {450.0 * 2.0});
    assert_resource_at!(resources : "nuyens" => Number 0.0);
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.nuyens" => Number {70000.0 - 9500.0 - 2500.0 - 2500.0 - 450.0 * 2.0 - 2500.0 * 4.0 - 100.0 - 4.0 * 200.0 - 10250.0 - 350.0 - 350.0 - 400.0});
}
