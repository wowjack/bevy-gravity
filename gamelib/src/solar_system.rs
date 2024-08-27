use bevy::{color::{palettes::{css::*, tailwind::*}, Srgba}, math::DVec2};
use crate::{gravity_system_tree::{builder::GravitySystemBuilder, dynamic_body::DynamicBody, static_body::{StaticBody, StaticPosition}}, math::get_orbital_speed};

use super::G;

const MASS_DIVISOR: f64 = 1e6;

// SOLAR SYSTEM ////////////////////////////////////////
pub const SUN_ORBITAL_RADIUS: f64 = 1e12;

pub const SUN_SYSTEM_RADIUS: f64 = 7e9;
pub const SUN_SYSTEM_TIME_STEP: u64 = 10;

pub const SUN_MU: f64 = /*10. **/ 1.9891e30*G/MASS_DIVISOR; // increased sun mass by factor of 10 to speed up orbital time
pub const SUN_RADIUS: f64 = 69340.;
pub const SUN_COLOR: Srgba = YELLOW;

pub fn solar_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(SUN_SYSTEM_RADIUS)
        .with_time_step(SUN_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, SUN_MU, SUN_RADIUS, SUN_COLOR.into()),
        ])
        .with_children(&[
            mercury_system().with_position(StaticPosition::Circular { radius: MERCURY_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MU, MERCURY_ORBITAL_RADIUS), start_angle: 0. }),
            venus_system().with_position(StaticPosition::Circular { radius: VENUS_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MU, VENUS_ORBITAL_RADIUS), start_angle: 0. }),
            earth_system().with_position(StaticPosition::Circular { radius: EARTH_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MU, EARTH_ORBITAL_RADIUS), start_angle: 0. }),
            mars_system().with_position(StaticPosition::Circular { radius: MARS_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MU, MARS_ORBITAL_RADIUS), start_angle: 0. }),
            jupiter_system().with_position(StaticPosition::Circular { radius: JUPITER_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MU, JUPITER_ORBITAL_RADIUS), start_angle: 0. }),
            saturn_system().with_position(StaticPosition::Circular { radius: SATURN_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MU, SATURN_ORBITAL_RADIUS), start_angle: 0. }),
            uranus_system().with_position(StaticPosition::Circular { radius: URANUS_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MU, URANUS_ORBITAL_RADIUS), start_angle: 0. }),
            neptune_system().with_position(StaticPosition::Circular { radius: NEPTUNE_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MU, NEPTUNE_ORBITAL_RADIUS), start_angle: 0. }),
            pluto_system().with_position(StaticPosition::Circular { radius: PLUTO_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MU, PLUTO_ORBITAL_RADIUS), start_angle: 0. }),
        ])
}
////////////////////////////////////////////////////////

// MERCURY SYSTEM //////////////////////////////////////
pub const MERCURY_ORBITAL_RADIUS: f64 = 54.28e6;

pub const MERCURY_SYSTEM_RADIUS: f64 = 5e5;
pub const MERCURY_SYSTEM_TIME_STEP: u64 = 1;

pub const MERCURY_MU: f64 = 3.285e23*G/MASS_DIVISOR;
pub const MERCURY_RADIUS: f64 = 2_439.7;
pub const MERCURY_COLOR: Srgba = DARK_GRAY;

pub fn mercury_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(MERCURY_SYSTEM_RADIUS)
        .with_time_step(MERCURY_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, MERCURY_MU, MERCURY_RADIUS, MERCURY_COLOR.into())
        ])
}
////////////////////////////////////////////////////////

// VENUS SYSTEM ////////////////////////////////////////
pub const VENUS_ORBITAL_RADIUS: f64 = 108.02e6;

pub const VENUS_SYSTEM_RADIUS: f64 = 5e5;
pub const VENUS_SYSTEM_TIME_STEP: u64 = 1;

pub const VENUS_MU: f64 = 4.867e24*G/MASS_DIVISOR;
pub const VENUS_RADIUS: f64 = 6051.8;
pub const VENUS_COLOR: Srgba = ORANGE_RED;

pub fn venus_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(VENUS_SYSTEM_RADIUS)
        .with_time_step(VENUS_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, VENUS_MU, VENUS_RADIUS, VENUS_COLOR.into())
        ])
}
////////////////////////////////////////////////////////

// EARTH SYSTEM ////////////////////////////////////////
pub const EARTH_ORBITAL_RADIUS: f64 = 1.5135e8;

pub const EARTH_SYSTEM_RADIUS: f64 = 1e6;
pub const EARTH_SYSTEM_TIME_STEP: u64 = 1;

pub const EARTH_MU: f64 = 5.972e24*G/MASS_DIVISOR;
pub const EARTH_RADIUS: f64 = 6378.14;
pub const EARTH_COLOR: Srgba = GREEN;

pub const MOON_ORBITAL_RADIUS: f64 = 384_400.;
pub const MOON_MU: f64 = 7.35e22*G/MASS_DIVISOR;
pub const MOON_RADIUS: f64 = 1737.4;
pub const MOON_COLOR: Srgba = WHITE;

pub fn earth_system() -> GravitySystemBuilder {
    let mut planet_orbiter = DynamicBody::new(DVec2::X*-7_000., DVec2::Y*get_orbital_speed(EARTH_MU, 7_000.)*7_000., 1e-30, 1., CORNFLOWER_BLUE.into());
    planet_orbiter.future_actions.extend((1650..1670).map(|x| (x, DVec2::Y)));
    planet_orbiter.future_actions.extend((1900..1920).map(|x| (x, DVec2::Y)));
    planet_orbiter.future_actions.extend((2250..2270).map(|x| (x, DVec2::Y)));
    planet_orbiter.future_actions.extend((2865..2885).map(|x| (x, DVec2::Y)));
    planet_orbiter.future_actions.extend((4470..4511).map(|x| (x, DVec2::Y)));
    planet_orbiter.future_actions.push_back((4511, DVec2::Y*0.59));
    planet_orbiter.future_actions.extend((7310..7350).map(|x| (x, DVec2::new(-1., -0.3))));
    planet_orbiter.future_actions.extend((7350..8000).map(|x| (x, DVec2::new(1., -0.25))));
    planet_orbiter.future_actions.extend((136_000..141_300).map(|x| (x, DVec2::new(-0.1, 0.))));
    planet_orbiter.future_actions.extend((178_000..178_200).map(|x| (x, DVec2::new(-0.1, 0.1))));
    planet_orbiter.future_actions.extend((178_665..178_700).map(|x| (x, DVec2::new(-0.41, 0.))));
    planet_orbiter.future_actions.extend((179_200..179_220).map(|x| (x, DVec2::new(1., -1.))));
    planet_orbiter.future_actions.extend((179_470..179_481).map(|x| (x, DVec2::new(0., -1.))));

    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(EARTH_SYSTEM_RADIUS)
        .with_time_step(EARTH_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, EARTH_MU, EARTH_RADIUS, EARTH_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: MOON_ORBITAL_RADIUS, speed: get_orbital_speed(EARTH_MU, MOON_ORBITAL_RADIUS), start_angle: 0. }, MOON_MU, MOON_RADIUS, MOON_COLOR.into()),
        ])
        .with_dynamic_bodies(&[
            planet_orbiter,
        ])
}
////////////////////////////////////////////////////////

// MARS SYSTEM /////////////////////////////////////////
pub const MARS_ORBITAL_RADIUS: f64 = 228e6;

pub const MARS_SYSTEM_RADIUS: f64 = 1e6;
pub const MARS_SYSTEM_TIME_STEP: u64 = 1;

pub const MARS_MU: f64 = 6.41693e23*G/MASS_DIVISOR;
pub const MARS_RADIUS: f64 = 3389.5;
pub const MARS_COLOR: Srgba = RED_50;

pub const PHOBOS_ORBITAL_RADIUS: f64 = 6_000.;
pub const PHOBOS_MU: f64 = 1.0659e16*G/MASS_DIVISOR;
pub const PHOBOS_RADIUS: f64 = 11.267;
pub const PHOBOS_COLOR: Srgba = GRAY_700;

pub const DEIMOS_ORBITAL_RADIUS: f64 = 23_460.;
pub const DEIMOS_MU: f64 = 1.51e15*G/MASS_DIVISOR;
pub const DEIMOS_RADIUS: f64 = 6.2;
pub const DEIMOS_COLOR: Srgba = GRAY_700;

pub fn mars_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(MARS_SYSTEM_RADIUS)
        .with_time_step(MARS_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, MARS_MU, MARS_RADIUS, MARS_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: PHOBOS_ORBITAL_RADIUS, speed: get_orbital_speed(MARS_MU, PHOBOS_ORBITAL_RADIUS), start_angle: 0. }, PHOBOS_MU, PHOBOS_RADIUS, PHOBOS_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: DEIMOS_ORBITAL_RADIUS, speed: get_orbital_speed(MARS_MU, DEIMOS_ORBITAL_RADIUS), start_angle: 0. }, DEIMOS_MU, DEIMOS_RADIUS, DEIMOS_COLOR.into()),
        ])
}
////////////////////////////////////////////////////////

// JUPITER SYSTEM //////////////////////////////////////
pub const JUPITER_ORBITAL_RADIUS: f64 = 7.78e8;

pub const JUPITER_SYSTEM_RADIUS: f64 = 3e6;
pub const JUPITER_SYSTEM_TIME_STEP: u64 = 1;

pub const JUPITER_MU: f64 = 1.898e27*G/MASS_DIVISOR;
pub const JUPITER_RADIUS: f64 = 69_911.;
pub const JUPITER_COLOR: Srgba = ORANGE;

pub const IO_ORBITAL_RADIUS: f64 = 422_000.;
pub const IO_MU: f64 = 8.9319e22*G/MASS_DIVISOR;
pub const IO_RADIUS: f64 = 1_821.6;
pub const IO_COLOR: Srgba = ALICE_BLUE;

pub const EUROPA_ORBITAL_RADIUS: f64 = 671_000.;
pub const EUROPA_MU: f64 = 4.799844e22*G/MASS_DIVISOR;
pub const EUROPA_RADIUS: f64 = 1_560.8;
pub const EUROPA_COLOR: Srgba = BLUE;

pub const CALLISTO_ORBITAL_RADIUS: f64 = 1_883_000.;
pub const CALLISTO_MU: f64 = 1.075938e23*G/MASS_DIVISOR;
pub const CALLISTO_RADIUS: f64 = 2_410.3;
pub const CALLISTO_COLOR: Srgba = YELLOW_100;

pub const GANYMEDE_ORBITAL_RADIUS: f64 = 1_070_000.;
pub const GANYMEDE_MU: f64 = 1.4819e23*G/MASS_DIVISOR;
pub const GANYMEDE_RADIUS: f64 = 2_634.1;
pub const GANYMEDE_COLOR: Srgba = GRAY;

pub fn jupiter_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(JUPITER_SYSTEM_RADIUS)
        .with_time_step(JUPITER_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, JUPITER_MU, JUPITER_RADIUS, JUPITER_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: IO_ORBITAL_RADIUS, speed: get_orbital_speed(JUPITER_MU, IO_ORBITAL_RADIUS), start_angle: 0. }, IO_MU, IO_RADIUS, IO_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: EUROPA_ORBITAL_RADIUS, speed: get_orbital_speed(JUPITER_MU, EUROPA_ORBITAL_RADIUS), start_angle: 0. }, EUROPA_MU, EUROPA_RADIUS, EUROPA_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: CALLISTO_ORBITAL_RADIUS, speed: get_orbital_speed(JUPITER_MU, CALLISTO_ORBITAL_RADIUS), start_angle: 0. }, CALLISTO_MU, CALLISTO_RADIUS, CALLISTO_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: GANYMEDE_ORBITAL_RADIUS, speed: get_orbital_speed(JUPITER_MU, GANYMEDE_ORBITAL_RADIUS), start_angle: 0. }, GANYMEDE_MU, GANYMEDE_RADIUS, GANYMEDE_COLOR.into()),
        ])
}
//////////////////////////////////////////////////////////

// SATURN SYSTEM /////////////////////////////////////////
pub const SATURN_ORBITAL_RADIUS: f64 = 1.4454e9;

pub const SATURN_SYSTEM_RADIUS: f64 = 6e6;
pub const SATURN_SYSTEM_TIME_STEP: u64 = 1;

pub const SATURN_MU: f64 = 5.68319e26*G/MASS_DIVISOR;
pub const SATURN_RADIUS: f64 = 58_232.;
pub const SATURN_COLOR: Srgba = LIGHT_GOLDENROD_YELLOW;

pub const MIMAS_ORBITAL_RADIUS: f64 = 185_539.;
pub const MIMAS_MU: f64 = 4e19*G/MASS_DIVISOR;
pub const MIMAS_RADIUS: f64 = 198.;
pub const MIMAS_COLOR: Srgba = DARK_GRAY;

pub const ENCELADUS_ORBITAL_RADIUS: f64 = 237_948.;
pub const ENCELADUS_MU: f64 = 1.1e20*G/MASS_DIVISOR;
pub const ENCELADUS_RADIUS: f64 = 252.;
pub const ENCELADUS_COLOR: Srgba = LIGHT_CORAL;

pub const TETHYS_ORBITAL_RADIUS: f64 = 294_619.;
pub const TETHYS_MU: f64 = 6.2e20*G/MASS_DIVISOR;
pub const TETHYS_RADIUS: f64 = 531.;
pub const TETHYS_COLOR: Srgba = LIGHT_SKY_BLUE;

pub const DIONE_ORBITAL_RADIUS: f64 = 377_396.;
pub const DIONE_MU: f64 = 1.1e21*G/MASS_DIVISOR;
pub const DIONE_RADIUS: f64 = 561.5;
pub const DIONE_COLOR: Srgba = LIGHT_GRAY;

pub const RHEA_ORBITAL_RADIUS: f64 = 527_108.;
pub const RHEA_MU: f64 = 2.3e21*G/MASS_DIVISOR;
pub const RHEA_RADIUS: f64 = 763.5;
pub const RHEA_COLOR: Srgba = GRAY;

pub const TITAN_ORBITAL_RADIUS: f64 = 1_221_870.;
pub const TITAN_MU: f64 = 1.35e23*G/MASS_DIVISOR;
pub const TITAN_RADIUS: f64 = 2_574.5;
pub const TITAN_COLOR: Srgba = YELLOW_50;

pub const LAPETUS_ORBITAL_RADIUS: f64 = 3_560_820.;
pub const LAPETUS_MU: f64 = 1.8e21*G/MASS_DIVISOR;
pub const LAPETUS_RADIUS: f64 = 735.;
pub const LAPETUS_COLOR: Srgba = DARK_GREEN;

pub fn saturn_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(SATURN_SYSTEM_RADIUS)
        .with_time_step(SATURN_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, SATURN_MU, SATURN_RADIUS, SATURN_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: MIMAS_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MU, MIMAS_ORBITAL_RADIUS), start_angle: 0. }, MIMAS_MU, MIMAS_RADIUS, MIMAS_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: ENCELADUS_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MU, ENCELADUS_ORBITAL_RADIUS), start_angle: 0. }, ENCELADUS_MU, ENCELADUS_RADIUS, ENCELADUS_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: TETHYS_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MU, TETHYS_ORBITAL_RADIUS), start_angle: 0. }, TETHYS_MU, TETHYS_RADIUS, TETHYS_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: DIONE_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MU, DIONE_ORBITAL_RADIUS), start_angle: 0. }, DIONE_MU, DIONE_RADIUS, DIONE_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: RHEA_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MU, RHEA_ORBITAL_RADIUS), start_angle: 0. }, RHEA_MU, RHEA_RADIUS, RHEA_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: TITAN_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MU, TITAN_ORBITAL_RADIUS), start_angle: 0. }, TITAN_MU, TITAN_RADIUS, TITAN_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: LAPETUS_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MU, LAPETUS_ORBITAL_RADIUS), start_angle: 0. }, LAPETUS_MU, LAPETUS_RADIUS, LAPETUS_COLOR.into()),
        ])
}
//////////////////////////////////////////////////////////

// URANUS SYSTEM /////////////////////////////////////////
pub const URANUS_ORBITAL_RADIUS: f64 = 2.9272e9;

pub const URANUS_SYSTEM_RADIUS: f64 = 6e6;
pub const URANUS_SYSTEM_TIME_STEP: u64 = 1;

pub const URANUS_MU: f64 = 8.618e25*G/MASS_DIVISOR;
pub const URANUS_RADIUS: f64 = 25_362.;
pub const URANUS_COLOR: Srgba = CORNFLOWER_BLUE;

pub const MIRANDA_ORBITAL_RADIUS: f64 = 129_390.;
pub const MIRANDA_MU: f64 = 6293e16*G/MASS_DIVISOR;
pub const MIRANDA_RADIUS: f64 = 235.8;
pub const MIRANDA_COLOR: Srgba = LIGHT_GRAY;

pub const ARIEL_ORBITAL_RADIUS: f64 = 191_020.;
pub const ARIEL_MU: f64 = 123_310e16*G/MASS_DIVISOR;
pub const ARIEL_RADIUS: f64 = 578.9;
pub const ARIEL_COLOR: Srgba = GRAY;

pub const UMBRIEL_ORBITAL_RADIUS: f64 = 266_000.;
pub const UMBRIEL_MU: f64 = 128_850e16*G/MASS_DIVISOR;
pub const UMBRIEL_RADIUS: f64 = 584.7;
pub const UMBRIEL_COLOR: Srgba = DARK_GRAY;

pub const TITANIA_ORBITAL_RADIUS: f64 = 436_300.;
pub const TITANIA_MU: f64 = 345_500e16*G/MASS_DIVISOR;
pub const TITANIA_RADIUS: f64 = 788.4;
pub const TITANIA_COLOR: Srgba = LIGHT_SALMON;

pub const OBERON_ORBITAL_RADIUS: f64 = 583_500.;
pub const OBERON_MU: f64 = 311_040e16*G/MASS_DIVISOR;
pub const OBERON_RADIUS: f64 = 761.4;
pub const OBERON_COLOR: Srgba = KHAKI;

pub fn uranus_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(URANUS_SYSTEM_RADIUS)
        .with_time_step(URANUS_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, URANUS_MU, URANUS_RADIUS, URANUS_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: MIRANDA_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MU, MIRANDA_ORBITAL_RADIUS), start_angle: 0. }, MIRANDA_MU, MIRANDA_RADIUS, MIRANDA_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: ARIEL_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MU, ARIEL_ORBITAL_RADIUS), start_angle: 0. }, ARIEL_MU, ARIEL_RADIUS, ARIEL_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: UMBRIEL_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MU, UMBRIEL_ORBITAL_RADIUS), start_angle: 0. }, UMBRIEL_MU, UMBRIEL_RADIUS, UMBRIEL_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: TITANIA_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MU, TITANIA_ORBITAL_RADIUS), start_angle: 0. }, TITANIA_MU, TITANIA_RADIUS, TITANIA_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: OBERON_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MU, OBERON_ORBITAL_RADIUS), start_angle: 0. }, OBERON_MU, OBERON_RADIUS, OBERON_COLOR.into()),
        ])
}
//////////////////////////////////////////////////////////

// NEPTUNE SYSTEM /////////////////////////////////////////
pub const NEPTUNE_ORBITAL_RADIUS: f64 = 4.4718e9;

pub const NEPTUNE_SYSTEM_RADIUS: f64 = 6e6;
pub const NEPTUNE_SYSTEM_TIME_STEP: u64 = 1;

pub const NEPTUNE_MU: f64 = 1.0241e26*G/MASS_DIVISOR;
pub const NEPTUNE_RADIUS: f64 = 24_622.;
pub const NEPTUNE_COLOR: Srgba = BLUE;

pub const DESPINA_ORBITAL_RADIUS: f64 = 27_700.;
pub const DESPINA_MU: f64 = 170e16*G/MASS_DIVISOR;
pub const DESPINA_RADIUS: f64 = 78.;
pub const DESPINA_COLOR: Srgba = LIGHT_GRAY;

pub const GALATEA_ORBITAL_RADIUS: f64 = 37_200.;
pub const GALATEA_MU: f64 = 280e16*G/MASS_DIVISOR;
pub const GALATEA_RADIUS: f64 = 140.;
pub const GALATEA_COLOR: Srgba = LIGHT_GRAY;

pub const LARISSA_ORBITAL_RADIUS: f64 = 48_800.;
pub const LARISSA_MU: f64 = 380e16*G/MASS_DIVISOR;
pub const LARISSA_RADIUS: f64 = 190.;
pub const LARISSA_COLOR: Srgba = LIGHT_GRAY;

pub const PROTEUS_ORBITAL_RADIUS: f64 = 117_647.;
pub const PROTEUS_MU: f64 = 3_900e16*G/MASS_DIVISOR;
pub const PROTEUS_RADIUS: f64 = 210.;
pub const PROTEUS_COLOR: Srgba = LIGHT_GRAY;

pub const TRITON_ORBITAL_RADIUS: f64 = 354_800.;
pub const TRITON_MU: f64 = 2_139_000e16*G/MASS_DIVISOR;
pub const TRITON_RADIUS: f64 = 1_352.6;
pub const TRITON_COLOR: Srgba = GRAY;

pub const NEREID_ORBITAL_RADIUS: f64 = 5_513_400.;
pub const NEREID_MU: f64 = 2400e16*G/MASS_DIVISOR;
pub const NEREID_RADIUS: f64 = 178.5;
pub const NEREID_COLOR: Srgba = LIGHT_GRAY;

pub fn neptune_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(NEPTUNE_SYSTEM_RADIUS)
        .with_time_step(NEPTUNE_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, NEPTUNE_MU, NEPTUNE_RADIUS, NEPTUNE_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: DESPINA_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MU, DESPINA_ORBITAL_RADIUS), start_angle: 0. }, DESPINA_MU, DESPINA_RADIUS, DESPINA_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: GALATEA_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MU, GALATEA_ORBITAL_RADIUS), start_angle: 0. }, GALATEA_MU, GALATEA_RADIUS, GALATEA_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: LARISSA_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MU, LARISSA_ORBITAL_RADIUS), start_angle: 0. }, LARISSA_MU, LARISSA_RADIUS, LARISSA_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: PROTEUS_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MU, PROTEUS_ORBITAL_RADIUS), start_angle: 0. }, PROTEUS_MU, PROTEUS_RADIUS, PROTEUS_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: TRITON_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MU, TRITON_ORBITAL_RADIUS), start_angle: 0. }, TRITON_MU, TRITON_RADIUS, TRITON_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: NEREID_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MU, NEREID_ORBITAL_RADIUS), start_angle: 0. }, NEREID_MU, NEREID_RADIUS, NEREID_COLOR.into()),
        ])
}
//////////////////////////////////////////////////////////

// PLUTO SYSTEM //////////////////////////////////////
pub const PLUTO_ORBITAL_RADIUS: f64 = 5.9e9;

pub const PLUTO_SYSTEM_RADIUS: f64 = 1e5;
pub const PLUTO_SYSTEM_TIME_STEP: u64 = 1;

pub const PLUTO_MU: f64 = 1.3e22*G/MASS_DIVISOR;
pub const PLUTO_RADIUS: f64 = 1_185.;
pub const PLUTO_COLOR: Srgba = LIGHT_SLATE_GREY;

pub const CHARON_ORBITAL_RADIUS: f64 = 19_640.;
pub const CHARON_MU: f64 = 158.7e19*G/MASS_DIVISOR;
pub const CHARON_RADIUS: f64 = 606.;
pub const CHARON_COLOR: Srgba = LIGHT_GRAY;

pub fn pluto_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(PLUTO_SYSTEM_RADIUS)
        .with_time_step(PLUTO_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, PLUTO_MU, PLUTO_RADIUS, PLUTO_COLOR.into()),
            StaticBody::new(StaticPosition::Circular { radius: CHARON_ORBITAL_RADIUS, speed: get_orbital_speed(CHARON_MU, CHARON_ORBITAL_RADIUS), start_angle: 0. }, CHARON_MU, CHARON_RADIUS, CHARON_COLOR.into()),
        ])
}
////////////////////////////////////////////////////////