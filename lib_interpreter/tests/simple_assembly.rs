#[macro_use]
mod utils;
use crate::utils::*;
use lib_interpreter::domain::resolution::ResolvedResourceValue;

const INDEX: &str = r#"
    {
        stats:
        {
            <? ./stats
                with con : 5
                with vol : 3 >
            <? ./magician
                with mag : 4
                with initiation : 1
                with trad : "vol" >
        }
    }
    "#;

const STATS: &str = r#"
    {
        con: $con
        vol: $vol
        ["con"]
        resist_phy: 0
        ["vol"]
        resist_ment: 0
        hit: 8
        ["vol" "con"]
        heal: 0
    }
    
    @TRANSFORM FINAL_STATS
    > $.resist_phy += $.con
    > $.resist_ment += $.vol
    > $.hit += floor($.con / 2)
    > $.heal += $.con + $.vol
    "#;

const MAGICIAN: &str = r#"
    {
        mag: $mag
        initiation: $initiation
        [$trad]
        resist_drain: 0
    }
    
    @TRANSFORM FINAL_STATS_END
    > $.resist_drain += floor($.$trad / 2)
    "#;

#[test]
fn it_shoud_assemble_from_different_files() {
    let (app, resolver, config) = MockedApp::new();

    config.borrow_mut().add_layer("FINAL_STATS");
    config.borrow_mut().add_layer("FINAL_STATS_END");
    resolver.borrow_mut().mock_file("./stats", STATS);
    resolver.borrow_mut().mock_file("./magician", MAGICIAN);

    let resolved_resources = app.assemble_from_str(INDEX);
    let resolved_resources = print_unwrap!(resolved_resources);

    assert_eq!(resolved_resources.len(), 9);

    let resource_0 = resolved_resources.get(0).unwrap();
    assert_resource!(resource_0: "stats.con", Number 5.0);

    let resource_1 = resolved_resources.get(1).unwrap();
    assert_resource!(resource_1: "stats.vol", Number 3.0);

    let resource_2 = resolved_resources.get(2).unwrap();
    assert_resource!(resource_2: "stats.resist_phy", Number 5.0);

    let resource_3 = resolved_resources.get(3).unwrap();
    assert_resource!(resource_3: "stats.resist_ment", Number 3.0);

    let resource_4 = resolved_resources.get(4).unwrap();
    assert_resource!(resource_4: "stats.hit", Number 10.0);

    let resource_5 = resolved_resources.get(5).unwrap();
    assert_resource!(resource_5: "stats.heal", Number 8.0);

    let resource_6 = resolved_resources.get(6).unwrap();
    assert_resource!(resource_6: "stats.mag", Number 4.0);

    let resource_7 = resolved_resources.get(7).unwrap();
    assert_resource!(resource_7: "stats.initiation", Number 1.0);

    let resource_8 = resolved_resources.get(8).unwrap();
    assert_resource!(resource_8: "stats.resist_drain", Number 1.0);

    let metas = &resource_8.metas;
    assert_eq!(metas.len(), 1);
    let first_meta = metas.get(0).unwrap();
    assert_resource!(first_meta: String "vol");
}
