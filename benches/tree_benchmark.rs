use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gamelib::{gravity_system_tree::{builder::GravitySystemBuilder, dynamic_body::DynamicBody, position_generator::PositionGenerator, static_body::{StaticBody, StaticPosition}, system_manager::GravitySystemManager, DVec2, Zero}, itertools::Itertools};
use rand::{thread_rng, Rng};
use gamelib::bevy::prelude::Entity;




fn single_layer_single_body_tree_benchmark(c: &mut Criterion) {
    let test_system = GravitySystemBuilder::new()
        .with_radius(1_000_000.)
        .with_position(StaticPosition::Still)
        .with_time_step(1)
        .with_static_bodies(&vec![
            StaticBody::new(StaticPosition::Still, 100., 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 400., speed: 0.001, start_angle: 0. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 500., speed: 0.0005, start_angle: 1. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 600., speed: 0.005, start_angle: 2. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 700., speed: 0.005, start_angle: 3. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 800., speed: 0.005, start_angle: 4. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 900., speed: 0.005, start_angle: 5. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 1000., speed: 0.005, start_angle: 6. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 1100., speed: 0.005, start_angle: 0.5 }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 1200., speed: 0.005, start_angle: 1.5 }, 0.00000000001, 1., None),
        ])
        .with_dynamic_bodies(&vec![
            DynamicBody::new(DVec2::new(20., 0.), DVec2::new(0., 2.5), 1., None),
        ])
        .build().expect("why did this fail?");


    c.bench_function("single body single layer", |b| b.iter(|| {
        let mut system = test_system.clone();
        // 100_000 ticks is about 28 mins at 20 text per second
        for _ in 0..100_000 {
            system.calculate_gravity();
        }
        let mut x = black_box(system);
        x.calculate_gravity();
    }));


}


fn two_layer_populated_tree_benchmark(c: &mut Criterion) {
    let test_system = GravitySystemBuilder::new()
        .with_radius(1_000.)
        .with_position(StaticPosition::Circular { radius: 100_000., speed: 0.0005, start_angle: 0. })
        .with_time_step(1)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, 100., 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 600., speed: 0.001, start_angle: 0. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 800., speed: 0.0005, start_angle: 0. }, 0.00000000001, 1., None),
            StaticBody::new(StaticPosition::Circular { radius: 500., speed: 0.005, start_angle: 0. }, 0.00000000001, 1., None),
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::new(10., 0.), DVec2::new(0., 3.), 1., None),
            DynamicBody::new(DVec2::new(20., 0.), DVec2::new(0., 2.5), 1., None),
            DynamicBody::new(DVec2::new(35., 0.), DVec2::new(0., 2.), 1., None),
            DynamicBody::new(DVec2::new(100., 0.), DVec2::new(0., 1.), 1., None),
            DynamicBody::new(DVec2::new(120., 0.), DVec2::new(0., 0.5), 1., None),

            DynamicBody::new(DVec2::new(-10., 0.), DVec2::new(0., 3.), 1., None),
            DynamicBody::new(DVec2::new(-20., 0.), DVec2::new(0., 2.5), 1., None),
            DynamicBody::new(DVec2::new(-35., 0.), DVec2::new(0., 2.), 1., None),
            DynamicBody::new(DVec2::new(-100., 0.), DVec2::new(0., 1.), 1., None),
            DynamicBody::new(DVec2::new(-120., 0.), DVec2::new(0., 0.5), 1., None),


            DynamicBody::new(DVec2::new(0., 10.), DVec2::new(3., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., 20.), DVec2::new(2.5, 0.), 1., None),
            DynamicBody::new(DVec2::new(0., 35.), DVec2::new(2., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., 100.), DVec2::new(1., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., 120.), DVec2::new(0.5, 0.), 1., None),

            DynamicBody::new(DVec2::new(0., -10.), DVec2::new(3., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., -20.), DVec2::new(2.5, 0.), 1., None),
            DynamicBody::new(DVec2::new(0., -35.), DVec2::new(2., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., -100.), DVec2::new(1., 0.), 1., None),
            DynamicBody::new(DVec2::new(0., -120.), DVec2::new(0.5, 0.), 1., None),
        ]);
    let parent_system = GravitySystemBuilder::new()
        .with_radius(1_000_000_000.)
        .with_position(StaticPosition::Still)
        .with_time_step(10)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, 1_000_000_000., 100., None)
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::new(51_000., 0.), DVec2::new(0., 140.), 1., None),
            DynamicBody::new(DVec2::new(40_000., 0.), DVec2::new(0., 160.), 1., None),
            DynamicBody::new(DVec2::new(42_000., 0.), DVec2::new(0., 140.), 1., None),
            DynamicBody::new(DVec2::new(43_000., 0.), DVec2::new(0., 140.), 1., None),
            DynamicBody::new(DVec2::new(45_000., 0.), DVec2::new(0., 150.), 1., None),
            DynamicBody::new(DVec2::new(50_000., 0.), DVec2::new(0., 150.), 1., None),
            DynamicBody::new(DVec2::new(52_000., 0.), DVec2::new(0., 140.), 1., None),
            DynamicBody::new(DVec2::new(56_000., 0.), DVec2::new(0., 120.), 1., None),
            DynamicBody::new(DVec2::new(58_000., 0.), DVec2::new(0., 120.), 1., None),
            DynamicBody::new(DVec2::new(60_000., 0.), DVec2::new(0., 100.), 1., None),

            DynamicBody::new(DVec2::new(-51_000., 0.), DVec2::new(0., -140.), 1., None),
            DynamicBody::new(DVec2::new(-40_000., 0.), DVec2::new(0., -160.), 1., None),
            DynamicBody::new(DVec2::new(-42_000., 0.), DVec2::new(0., -140.), 1., None),
            DynamicBody::new(DVec2::new(-43_000., 0.), DVec2::new(0., -140.), 1., None),
            DynamicBody::new(DVec2::new(-45_000., 0.), DVec2::new(0., -150.), 1., None),
            DynamicBody::new(DVec2::new(-50_000., 0.), DVec2::new(0., -150.), 1., None),
            DynamicBody::new(DVec2::new(-52_000., 0.), DVec2::new(0., -140.), 1., None),
            DynamicBody::new(DVec2::new(-56_000., 0.), DVec2::new(0., -120.), 1., None),
            DynamicBody::new(DVec2::new(-58_000., 0.), DVec2::new(0., -120.), 1., None),
            DynamicBody::new(DVec2::new(-60_000., 0.), DVec2::new(0., -100.), 1., None),
        ])
        .with_children(&[
            test_system.clone().with_position(StaticPosition::Circular { radius: 105_000., speed: 0.00045, start_angle: 0.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 160_000., speed: 0.0003, start_angle: 3.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 117_000., speed: 0.00044, start_angle: 5. }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 122_000., speed: 0.00053, start_angle: 1.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 136_000., speed: 0.00044, start_angle: 0.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 140_000., speed: 0.00035, start_angle: 2. }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 100_000., speed: 0.0005, start_angle: 3. }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 100_000., speed: 0.0005, start_angle: 5.5 }),
            test_system.clone().with_position(StaticPosition::Circular { radius: 110_000., speed: 0.0005, start_angle: 4. }),
            test_system,
        ]);
    let entities = (0..parent_system.total_bodies()).map(|i| Entity::from_raw(u32::MAX-i as u32)).collect_vec();
    let mut manager = GravitySystemManager::new(parent_system, &entities);
    let mut time = 1;

    c.bench_function("two layer populated tree", |b| b.iter(|| {
        manager.get_state_at_time(time);
        time += 1;
    }));
    black_box(manager);
}


criterion_group!(benches,
    single_layer_single_body_tree_benchmark,
    two_layer_populated_tree_benchmark
);
criterion_main!(benches);