use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gamelib::gravity_system_tree::{builder::GravitySystemBuilder, dynamic_body::DynamicBody, position_generator::PositionGenerator, static_body::{StaticBody, StaticPosition}, DVec2, Zero};
use rand::{thread_rng, Rng};




fn tree_single_layer_benchmark(c: &mut Criterion) {
    let test_system = GravitySystemBuilder::new()
        .with_radius(1_000_000.)
        .with_position(StaticPosition::Still)
        .with_time_step(1)
        .with_static_bodies(&vec![
            StaticBody::new(StaticPosition::Still, 100., 1., None),
        ])
        .with_dynamic_bodies(&vec![
            DynamicBody::new(DVec2::new(10., 0.), DVec2::new(0., 10.), 1., None),
            DynamicBody::new(DVec2::new(20., 0.), DVec2::new(0., 5.), 1., None),
            DynamicBody::new(DVec2::new(35., 0.), DVec2::new(0., 2.), 1., None),
            DynamicBody::new(DVec2::new(100., 0.), DVec2::new(0., 1.), 1., None),
            DynamicBody::new(DVec2::new(120., 0.), DVec2::new(0., 1.5), 1., None),
            DynamicBody::new(DVec2::new(-10., 0.), DVec2::new(0., 10.), 1., None),
            DynamicBody::new(DVec2::new(-20., 0.), DVec2::new(0., 5.), 1., None),
            DynamicBody::new(DVec2::new(-35., 0.), DVec2::new(0., 2.), 1., None),
            DynamicBody::new(DVec2::new(-100., 0.), DVec2::new(0., 1.), 1., None),
            DynamicBody::new(DVec2::new(-120., 0.), DVec2::new(0., 1.5), 1., None),
        ])
        .build().expect("why did this fail?");


    c.bench_function("position generator", |b| b.iter(|| {
        let mut system = test_system.clone();
        for _ in 0..100 {
            system.calculate_gravity();
        }
        let mut x = black_box(system);
        x.calculate_gravity();
    }));


}


criterion_group!(benches, tree_single_layer_benchmark);
criterion_main!(benches);