use crate::utils::*;
use custom_dsl::domain::resolution::{ResolvedResource, ResolvedResourceValue};
use files_shrimp::*;
use regex::Regex;

#[macro_use]
mod utils;
mod files_shrimp;

const INDEX: &str = r#"
    {
        name: Shrimp

        <? ./metatypes/human >
        tags! "13 ans"
        tags! technorigger

        knowledges! Jeux tridéos
        knowledges! Séries tridéos
        knowledges! Drônes

        nuyens: 70000

        identities:
        {
            <! ./identity/legacy
                with <contacts! ./identity/contact
                    with name: "D-Boss"
                    with loyalty: 4
                    with connection: 4
                    with description: "Decker fan de complot">
                with <contacts! ./identity/contact
                    with name: "Terrance"
                    with loyalty: 3
                    with connection: 2
                    with description: "Ouvrier de casse militaire d'ARES">
            >
            <! ./identity/fake
                with name: "Laurence Guinvite"
                with quality: 4
                with <? ./utils/transfer_all_nuyens >
                with <lifestyles! ./identity/lifestyle/squat >
                with <licences! ./licence
                    with name: "Concierge de chantier"
                    with description: ""
                    with quality: 4 
                >
                
            >
        }

        stats:
        {
            <? ./stats/base
                with con : 1
                with agi : 4
                with rea : 2
                with for : 1
                with vol : 4
                with log : 6
                with int : 4
                with cha : 2
                with ess : 6 >
            <? ./stats/techno
                with res : 7
                with submersion : 1 >
        }

        skills:
        {
            <combat rapproché? ./skill with score: 1>
            <perception? ./skill with score: 1>
            <furtivité? ./skill with score: 1>
            <athlétisme? ./skill with score: 3 >
            <électronique? ./skill with score: 4>
            <ingénierie? ./skill
                with score: 6
                with <? ./skill/spec with name: "Artillerie" >
            >
            <pilotage? ./skill
                with score: 6
                with <? ./skill/spec with name: "Appareils aux sols" >
            >
            <technomancie? ./skill
                with score: 6
                with <? ./skill/spec with name: "Compilation" >
                with <? ./skill/mast with name: "Inscription" >
            >
        }

        traits:
        {
            <@ ./traits/bricoleur_prevoyant >
            <@ ./traits/ami_des_sprites with type: "machine" >
            <@ ./traits/paralysie_du_combat>
            <@ ./traits/rhinite_chronique>
        }

        inventory:
        {
            <@ ./drones/crawler
                with <? ./drones_mods/monture >
                with <? ./drones_mods/monture >
            >

            <@ ./drones/kanmushi
                with <? ./utils/quantity with q: 2 >
            >
        }
    }
"#;

/*

<! identity/fake
                with name: "Laurence Guinvite"
                with <? identity/lifestyle/squat >


            >
*/

/*
<! ./identity/legacy
                with <? ./identity/contact
                    with name: "D-Boss"
                    with loyalty: 4
                    with connection: 4
                    with description: "Decker fan de complot">
                >
                with <? ./identity/contact
                    with name: "Terrance"
                    with loyalty: 3
                    with connection: 2
                    with description: "Ouvrier de casse militaire d'ARES">
                >
            >
*/

#[test]
fn it_shoud_assemble_shrimp() {
    let (app, resolver, config) = MockedApp::new();

    config.borrow_mut().add_layer("FINAL_STATS");
    config.borrow_mut().add_layer("FINAL_STATS_END");
    config.borrow_mut().add_layer("BUY_FINAL");
    config.borrow_mut().add_layer("BANK");

    let mocks = [
        ("./stats/base", STATS_BASE),
        ("./stats/techno", STATS_TECHNO),
        ("./drones/rules", DRONE_RULES),
        ("./drones/crawler", DRONE_CRAWLER),
        ("./drones/kanmushi", DRONE_KANMUSHI),
        ("./drones_mods/monture", DRONE_MOD_MONTURE),
        ("./utils/buy", UTILS_BUY),
        ("./utils/quantity", UTILS_QUANTITY),
        ("./utils/transfer_all_nuyens", UTILS_TRANSFER_ALL_NUYENS),
        ("./metatypes/human", METATYPE_HUMAN),
        ("./skill", SKILL),
        ("./skill/spec", SKILL_SPEC),
        ("./skill/mast", SKILL_MAST),
        ("./traits/ami_des_sprites", TRAIT_AMI_DES_SPRITES),
        ("./traits/bricoleur_prevoyant", TRAIT_BRICOLEUR_PREVOYANT),
        ("./traits/paralysie_du_combat", TRAIT_PARALYSIE_DU_COMBAT),
        ("./traits/rhinite_chronique", TRAIT_RHINITE_CHRONIQUE),
        ("./identity/contact", IDENTITY_CONTACT),
        ("./identity/legacy", IDENTITY_LEGACY),
        ("./identity/fake", IDENTITY_FAKE),
        ("./identity/lifestyle/squat", IDENTITY_LIFESTYLE_SQUAT),
        ("./licence", LICENCE),
    ];
    mocks.iter().for_each(|(path, content)| {
        resolver.borrow_mut().mock_file(path, content);
    });

    let resources = app.assemble_from_str(INDEX);
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
    assert_resource_at!(resources : "skills.combat rapproché.score" => Number 1.0);
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
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.quality" => String "i4");
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.lifestyles.[a-zA-Z0-9]+.name" => String "squatteur");

    /* Testing licences */
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.licences.[a-zA-Z0-9]+.name" => String "Concierge de chantier");
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.licences.[a-zA-Z0-9]+.description" => String "");
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.licences.[a-zA-Z0-9]+.quality" => String "i4");

    /* Testing traits */
    assert_resource_at!(resources : "traits.ami des sprites.description" => String "__A1__ lorsque vous compilez ou inscrivez un sprite machine.");
    assert_resource_at!(resources : "traits.bricoleur prévoyant.description" => String "__A1__ lorsque vous utilisez une machine que vous avez bricolé.");
    assert_resource_at!(resources : "traits.paralysie du combat.description" => String "Au premier round, vous ne pouvez pas vous déplacer et vous jouez en dernier.");
    assert_resource_at!(resources : "traits.rhinite chronique.description" => String "Vous éternuez souvent. __D1__ lors des tests de discrétion.");

    /* Testing tags */
    let tags_regex = Regex::new(r#"tags\.([a-zA-Z0-9]+)"#).unwrap();
    let tags = resources
        .iter()
        .filter(|ResolvedResource { identifier, .. }| match identifier {
            Some(i) => tags_regex.is_match(i),
            _ => false,
        })
        .collect::<Vec<_>>();
    assert_eq!(tags.len(), 3);

    let first_tag = *tags.get(1).unwrap();
    let value = as_variant!(&first_tag.value, ResolvedResourceValue::String);
    assert_eq!(value, "13 ans");
    let second_tag = tags.get(2).unwrap();
    let value = as_variant!(&second_tag.value, ResolvedResourceValue::String);
    assert_eq!(value, "technorigger");

    /* Testing drone rules */
    assert_resource_at!(resources : "inventory.Crawler.stats.hit" => Number 11.0);

    /* Testing drone mods */
    let crawler_slot_regex =
        Regex::new(r#"inventory\.Crawler\.slots\.([a-zA-Z0-9]+)\.name"#).unwrap();
    let crawler_slots = resources
        .iter()
        .filter(|ResolvedResource { identifier, .. }| match identifier {
            Some(i) => crawler_slot_regex.is_match(i),
            _ => false,
        })
        .collect::<Vec<_>>();
    assert_eq!(crawler_slots.len(), 2);

    /* Testing buy util */
    assert_resource_at!(resources : "inventory.Crawler.price" => Number {9500.0 + 2500.0 + 2500.0});
    assert_resource_at!(resources : "inventory.Kanmushi.price" => Number {450.0 * 2.0});
    assert_resource_at!(resources : "nuyens" => Number 0.0);
    assert_resource_at!(resources : "identities.[a-zA-Z0-9]+.nuyens" => Number {70000.0 - 9500.0 - 2500.0 - 2500.0 - 450.0 * 2.0 - 2500.0 * 4.0 - 100.0 - 4.0 * 200.0});
}
