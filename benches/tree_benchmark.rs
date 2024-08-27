use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gamelib::{bevy::{color::palettes::css::WHITE, math::DVec2}, gravity_system_tree::{builder::GravitySystemBuilder, dynamic_body::DynamicBody, static_body::{StaticBody, StaticPosition}, system_manager::GravitySystemManager}, itertools::Itertools, math::*, G};
use gamelib::bevy::prelude::Entity;




fn single_layer_single_body_tree_benchmark(c: &mut Criterion) {
    let test_system = GravitySystemBuilder::new()
        .with_radius(1_000_000.)
        .with_position(StaticPosition::Still)
        .with_time_step(1)
        .with_static_bodies(&vec![
            StaticBody::new(StaticPosition::Still, 100., 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 400., speed: 0.001, start_angle: 0. }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 500., speed: 0.0005, start_angle: 1. }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 600., speed: 0.005, start_angle: 2. }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 700., speed: 0.005, start_angle: 3. }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 800., speed: 0.005, start_angle: 4. }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 900., speed: 0.005, start_angle: 5. }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 1000., speed: 0.005, start_angle: 6. }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 1100., speed: 0.005, start_angle: 0.5 }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 1200., speed: 0.005, start_angle: 1.5 }, 0.00000000001, 1., WHITE.into()),
        ])
        .with_dynamic_bodies(&vec![
            DynamicBody::new(DVec2::new(20., 0.), DVec2::new(0., 2.5), 1., 1., WHITE.into()),
        ]);
    
    let manager = GravitySystemManager::new(test_system);


    c.bench_function("single body single layer", |b| b.iter(|| {
        let mut system = manager.clone();
        for _ in 0..100_000 {
            system.step();
        }
        black_box(system);
    }));
}


fn two_layer_populated_tree_benchmark(c: &mut Criterion) {
    let test_system = GravitySystemBuilder::new()
        .with_radius(1_000.)
        .with_position(StaticPosition::Circular { radius: 100_000., speed: 0.0005, start_angle: 0. })
        .with_time_step(1)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, 100., 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 600., speed: 0.001, start_angle: 0. }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 800., speed: 0.0005, start_angle: 0. }, 0.00000000001, 1., WHITE.into()),
            StaticBody::new(StaticPosition::Circular { radius: 500., speed: 0.005, start_angle: 0. }, 0.00000000001, 1., WHITE.into()),
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::new(10., 0.), DVec2::new(0., 3.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(20., 0.), DVec2::new(0., 2.5), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(35., 0.), DVec2::new(0., 2.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(100., 0.), DVec2::new(0., 1.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(120., 0.), DVec2::new(0., 0.5), 1., 1., WHITE.into()),

            DynamicBody::new(DVec2::new(-10., 0.), DVec2::new(0., 3.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-20., 0.), DVec2::new(0., 2.5), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-35., 0.), DVec2::new(0., 2.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-100., 0.), DVec2::new(0., 1.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-120., 0.), DVec2::new(0., 0.5), 1., 1., WHITE.into()),


            DynamicBody::new(DVec2::new(0., 10.), DVec2::new(3., 0.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(0., 20.), DVec2::new(2.5, 0.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(0., 35.), DVec2::new(2., 0.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(0., 100.), DVec2::new(1., 0.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(0., 120.), DVec2::new(0.5, 0.), 1., 1., WHITE.into()),

            DynamicBody::new(DVec2::new(0., -10.), DVec2::new(3., 0.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(0., -20.), DVec2::new(2.5, 0.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(0., -35.), DVec2::new(2., 0.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(0., -100.), DVec2::new(1., 0.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(0., -120.), DVec2::new(0.5, 0.), 1., 1., WHITE.into()),
        ]);
    let parent_system = GravitySystemBuilder::new()
        .with_radius(1_000_000_000.)
        .with_position(StaticPosition::Still)
        .with_time_step(10)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, 1_000_000_000., 100., WHITE.into())
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::new(51_000., 0.), DVec2::new(0., 140.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(40_000., 0.), DVec2::new(0., 160.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(42_000., 0.), DVec2::new(0., 140.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(43_000., 0.), DVec2::new(0., 140.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(45_000., 0.), DVec2::new(0., 150.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(50_000., 0.), DVec2::new(0., 150.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(52_000., 0.), DVec2::new(0., 140.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(56_000., 0.), DVec2::new(0., 120.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(58_000., 0.), DVec2::new(0., 120.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(60_000., 0.), DVec2::new(0., 100.), 1., 1., WHITE.into()),

            DynamicBody::new(DVec2::new(-51_000., 0.), DVec2::new(0., -140.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-40_000., 0.), DVec2::new(0., -160.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-42_000., 0.), DVec2::new(0., -140.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-43_000., 0.), DVec2::new(0., -140.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-45_000., 0.), DVec2::new(0., -150.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-50_000., 0.), DVec2::new(0., -150.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-52_000., 0.), DVec2::new(0., -140.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-56_000., 0.), DVec2::new(0., -120.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-58_000., 0.), DVec2::new(0., -120.), 1., 1., WHITE.into()),
            DynamicBody::new(DVec2::new(-60_000., 0.), DVec2::new(0., -100.), 1., 1., WHITE.into()),
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
    let manager = GravitySystemManager::new(parent_system);

    c.bench_function("two layer populated tree", |b| b.iter(|| {
        let mut manager = manager.clone();
        for _ in 0..100_000 {
            manager.step();
        }
    }));
    black_box(manager);
}


fn deep_tree_single_body(c: &mut Criterion) {
    let galaxy_mu = 1e33*G/100.;
    let galaxy_system_radius = 1e20;
    let galaxy_system_time_step = 100;
    let galaxy_radius = 1000.;
    let galaxy_color = WHITE.into();

    let stellar_orbital_radius = 1e12;
    let stellar_mu = 1.9891e30*G/100.;
    let stellar_system_radius = 5e9;
    let stellar_system_time_step = 10;
    let stellar_radius = 100.;
    let stellar_color = WHITE.into();

    let planet_orbital_radius = 1.5135e8;
    let planet_mu = 5.972e24*G/100.;
    let planet_system_radius = 5e6;
    let planet_system_time_step = 1;
    let planet_radius = 10.;
    let planet_color = WHITE.into();

    let moon_orbital_radius = 384_400.;
    let moon_mu = 7.35e22*G/100.;
    let moon_radius = 1.;
    let moon_color = WHITE.into();


    let planet_system = GravitySystemBuilder::new()
        .with_radius(planet_system_radius)
        .with_position(StaticPosition::Circular { radius: planet_orbital_radius, speed: get_orbital_speed(stellar_mu, planet_orbital_radius), start_angle: 0. })
        .with_time_step(planet_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, planet_mu, planet_radius, planet_color),
            StaticBody::new(StaticPosition::Circular { radius: moon_orbital_radius, speed: get_orbital_speed(planet_mu, moon_orbital_radius), start_angle: 0. }, moon_mu, moon_radius, moon_color),
        ])
        .with_dynamic_bodies(&[
            DynamicBody::new(DVec2::X*-100_000., 1.3*DVec2::Y*get_orbital_speed(planet_mu, 100_000.)*100_000., 1e-20, 1., WHITE.into()),
        ]);
    let stellar_system = GravitySystemBuilder::new()
        .with_radius(stellar_system_radius)
        .with_position(StaticPosition::Circular { radius: stellar_orbital_radius, speed: get_orbital_speed(galaxy_mu, stellar_orbital_radius), start_angle: 0. })
        .with_time_step(stellar_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, stellar_mu, stellar_radius, stellar_color),
        ])
        .with_children(&[planet_system]);
    let galactic_system = GravitySystemBuilder::new()
        .with_radius(galaxy_system_radius)
        .with_position(StaticPosition::Still)
        .with_time_step(galaxy_system_time_step)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, galaxy_mu, galaxy_radius, galaxy_color)
        ])
        .with_children(&[stellar_system]);


    let system = GravitySystemManager::new(galactic_system);

    c.bench_function("deep tree single body", |b| b.iter(|| {
        let mut system = system.clone();
        for _ in 0..100_000 {
            system.step();
        }
    }));
    black_box(system);
}




criterion_group!(benches,
    single_layer_single_body_tree_benchmark,
    two_layer_populated_tree_benchmark,
    deep_tree_single_body
);
criterion_main!(benches);