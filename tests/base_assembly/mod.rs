#[test]
fn it_shoud_assemble_from_different_files() {
    use crate::utils::get_test_app;
    panic!("TODO");

    /* let (app, output) = get_test_app();

    app.assemble("./index.ttl");

    let ouputed_file = output.get_in_memory("index.yml");

    assert_eq!(
        ouputed_file,
        r#"
            con:
                value: 2
            vol:
                value: 3
            resist_phy:
                value: 2
                meta:
                    - "con"
            resist_ment:
                value: 3
                meta:
                    - "vol"
            hit:
                value: 9
            heal:
                value: 5
                meta:
                    - "con"
                    - "vol"
            mag:
                value: 4
            initiation:
                value: 1
            resist_drain:
                value: 2
                meta:
                    - "int"
        "#
    );

    assert_eq!(1, 2); */
}
