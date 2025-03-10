use tests_client::*;

async fn batch_get_by_key(s: impl Setup) {
    schema!(
        "
        model Foo {
            #[key]
            one: String,

            #[key]
            two: String,
        }"
    );

    let db = s.setup(db::load_schema()).await;
    let mut keys = vec![];

    for i in 0..5 {
        #[allow(clippy::disallowed_names)]
        let foo = db::Foo::create()
            .one(format!("foo-{i}"))
            .two(format!("bar-{i}"))
            .exec(&db)
            .await
            .unwrap();

        keys.push((foo.one.clone(), foo.two.clone()));
    }

    let foos: Vec<_> = db::Foo::filter_by_one_and_two_batch(&[
        (&keys[0].0, &keys[0].1),
        (&keys[1].0, &keys[1].1),
        (&keys[2].0, &keys[2].1),
    ])
    .collect(&db)
    .await
    .unwrap();

    assert_eq!(3, foos.len());

    #[allow(clippy::disallowed_names)]
    for foo in foos {
        assert!(keys.iter().any(|key| foo.one == key.0 && foo.two == key.1));
    }
}

tests!(batch_get_by_key,);
