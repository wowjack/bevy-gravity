use bevy::{color::{palettes::{css::*, tailwind::*}, Srgba}, math::DVec2};
use crate::{gravity_system_tree::{builder::GravitySystemBuilder, dynamic_body::DynamicBody, static_body::{StaticBody, StaticPosition}}, math::get_orbital_speed};

use super::G;

const MASS_DIVISOR: f64 = 1.;//e6;

// SOLAR SYSTEM ////////////////////////////////////////
pub const SUN_ORBITAL_RADIUS: f64 = 2.45979e14;

pub const SUN_SYSTEM_RADIUS: f64 = 7e9;
pub const SUN_SYSTEM_TIME_STEP: u64 = 10;

pub const SUN_MASS: f64 = 1.9891e30/MASS_DIVISOR; // increased sun mass by factor of 10 to speed up orbital time
pub const SUN_RADIUS: f64 = 69340.;
pub const SUN_COLOR: Srgba = YELLOW;
pub const SUN_NAME: &str = "Sun";

pub fn solar_system() -> GravitySystemBuilder {
    let sun_orbiter = DynamicBody::new(DVec2::X*100_000_000., /*1.402**/DVec2::Y*get_orbital_speed(SUN_MASS, 100_000_000.)*100_000_000., 1e-30, 1., CORNFLOWER_BLUE.into(), "Satellite".into());
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(SUN_SYSTEM_RADIUS)
        .with_time_step(SUN_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, SUN_MASS, SUN_RADIUS, SUN_COLOR.into(), SUN_NAME.into()),
        ])
        .with_children(&[
            mercury_system().with_position(StaticPosition::Circular { radius: MERCURY_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MASS, MERCURY_ORBITAL_RADIUS), start_angle: 0. }),
            venus_system().with_position(StaticPosition::Circular { radius: VENUS_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MASS, VENUS_ORBITAL_RADIUS), start_angle: 0. }),
            earth_system().with_position(StaticPosition::Circular { radius: EARTH_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MASS, EARTH_ORBITAL_RADIUS), start_angle: 0. }),
            mars_system().with_position(StaticPosition::Circular { radius: MARS_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MASS, MARS_ORBITAL_RADIUS), start_angle: 0. }),
            jupiter_system().with_position(StaticPosition::Circular { radius: JUPITER_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MASS, JUPITER_ORBITAL_RADIUS), start_angle: 0. }),
            saturn_system().with_position(StaticPosition::Circular { radius: SATURN_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MASS, SATURN_ORBITAL_RADIUS), start_angle: 0. }),
            uranus_system().with_position(StaticPosition::Circular { radius: URANUS_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MASS, URANUS_ORBITAL_RADIUS), start_angle: 0. }),
            neptune_system().with_position(StaticPosition::Circular { radius: NEPTUNE_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MASS, NEPTUNE_ORBITAL_RADIUS), start_angle: 0. }),
            pluto_system().with_position(StaticPosition::Circular { radius: PLUTO_ORBITAL_RADIUS, speed: get_orbital_speed(SUN_MASS, PLUTO_ORBITAL_RADIUS), start_angle: 0. }),
        ])
        .with_dynamic_bodies(&[
            //sun_orbiter
        ])
}
////////////////////////////////////////////////////////

// MERCURY SYSTEM //////////////////////////////////////
pub const MERCURY_ORBITAL_RADIUS: f64 = 54.28e6;

pub const MERCURY_SYSTEM_RADIUS: f64 = 5e5;
pub const MERCURY_SYSTEM_TIME_STEP: u64 = 1;

pub const MERCURY_MASS: f64 = 3.285e23/MASS_DIVISOR;
pub const MERCURY_RADIUS: f64 = 2_439.7;
pub const MERCURY_COLOR: Srgba = DARK_GRAY;
pub const MERCURY_NAME: &str = "Mercury";

pub fn mercury_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(MERCURY_SYSTEM_RADIUS)
        .with_time_step(MERCURY_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, MERCURY_MASS, MERCURY_RADIUS, MERCURY_COLOR.into(), MERCURY_NAME.into())
        ])
}
////////////////////////////////////////////////////////

// VENUS SYSTEM ////////////////////////////////////////
pub const VENUS_ORBITAL_RADIUS: f64 = 108.02e6;

pub const VENUS_SYSTEM_RADIUS: f64 = 5e5;
pub const VENUS_SYSTEM_TIME_STEP: u64 = 1;

pub const VENUS_MASS: f64 = 4.867e24/MASS_DIVISOR;
pub const VENUS_RADIUS: f64 = 6051.8;
pub const VENUS_COLOR: Srgba = ORANGE_RED;
pub const VENUS_NAME: &str = "Venus";

pub fn venus_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(VENUS_SYSTEM_RADIUS)
        .with_time_step(VENUS_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, VENUS_MASS, VENUS_RADIUS, VENUS_COLOR.into(), VENUS_NAME.into())
        ])
}
////////////////////////////////////////////////////////

// EARTH SYSTEM ////////////////////////////////////////
pub const EARTH_ORBITAL_RADIUS: f64 = 1.5135e8;

pub const EARTH_SYSTEM_RADIUS: f64 = 1e6;
pub const EARTH_SYSTEM_TIME_STEP: u64 = 1;

pub const EARTH_MASS: f64 = 5.972e24/MASS_DIVISOR;
pub const EARTH_RADIUS: f64 = 6378.14;
pub const EARTH_COLOR: Srgba = GREEN;
pub const EARTH_NAME: &str = "Earth";

pub const MOON_ORBITAL_RADIUS: f64 = 384_400.;
pub const MOON_MASS: f64 = 7.35e22/MASS_DIVISOR;
pub const MOON_RADIUS: f64 = 1737.4;
pub const MOON_COLOR: Srgba = WHITE;
pub const MOON_NAME: &str = "Moon";

pub fn earth_system() -> GravitySystemBuilder {
    let planet_orbiter = DynamicBody::new(DVec2::NEG_Y*7_000., 1.402*DVec2::X*get_orbital_speed(EARTH_MASS, 7_000.)*7_000., 1e-30, 1., CORNFLOWER_BLUE.into(), "Satellite".into());    //planet_orbiter.future_actions.extend((2300..3000).map(|x| (x, DVec2::Y)));
    

    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(EARTH_SYSTEM_RADIUS)
        .with_time_step(EARTH_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, EARTH_MASS, EARTH_RADIUS, EARTH_COLOR.into(), EARTH_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: MOON_ORBITAL_RADIUS, speed: get_orbital_speed(EARTH_MASS, MOON_ORBITAL_RADIUS), start_angle: 0. }, MOON_MASS, MOON_RADIUS, MOON_COLOR.into(), MOON_NAME.into()),
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

pub const MARS_MASS: f64 = 6.41693e23/MASS_DIVISOR;
pub const MARS_RADIUS: f64 = 3389.5;
pub const MARS_COLOR: Srgba = RED_50;
pub const MARS_NAME: &str = "Mars";

pub const PHOBOS_ORBITAL_RADIUS: f64 = 6_000.;
pub const PHOBOS_MASS: f64 = 1.0659e16/MASS_DIVISOR;
pub const PHOBOS_RADIUS: f64 = 11.267;
pub const PHOBOS_COLOR: Srgba = GRAY_700;
pub const PHOBOS_NAME: &str = "Phobos";

pub const DEIMOS_ORBITAL_RADIUS: f64 = 23_460.;
pub const DEIMOS_MASS: f64 = 1.51e15/MASS_DIVISOR;
pub const DEIMOS_RADIUS: f64 = 6.2;
pub const DEIMOS_COLOR: Srgba = GRAY_700;
pub const DEIMOS_NAME: &str = "Deimos";

pub fn mars_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(MARS_SYSTEM_RADIUS)
        .with_time_step(MARS_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, MARS_MASS, MARS_RADIUS, MARS_COLOR.into(), MARS_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: PHOBOS_ORBITAL_RADIUS, speed: get_orbital_speed(MARS_MASS, PHOBOS_ORBITAL_RADIUS), start_angle: 0. }, PHOBOS_MASS, PHOBOS_RADIUS, PHOBOS_COLOR.into(), PHOBOS_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: DEIMOS_ORBITAL_RADIUS, speed: get_orbital_speed(MARS_MASS, DEIMOS_ORBITAL_RADIUS), start_angle: 0. }, DEIMOS_MASS, DEIMOS_RADIUS, DEIMOS_COLOR.into(), DEIMOS_NAME.into()),
        ])
}
////////////////////////////////////////////////////////

// JUPITER SYSTEM //////////////////////////////////////
pub const JUPITER_ORBITAL_RADIUS: f64 = 7.78e8;

pub const JUPITER_SYSTEM_RADIUS: f64 = 3e6;
pub const JUPITER_SYSTEM_TIME_STEP: u64 = 1;

pub const JUPITER_MASS: f64 = 1.898e27/MASS_DIVISOR;
pub const JUPITER_RADIUS: f64 = 69_911.;
pub const JUPITER_COLOR: Srgba = ORANGE;
pub const JUPITER_NAME: &str = "Jupiter";

pub const IO_ORBITAL_RADIUS: f64 = 422_000.;
pub const IO_MASS: f64 = 8.9319e22/MASS_DIVISOR;
pub const IO_RADIUS: f64 = 1_821.6;
pub const IO_COLOR: Srgba = ALICE_BLUE;
pub const IO_NAME: &str = "Io";

pub const EUROPA_ORBITAL_RADIUS: f64 = 671_000.;
pub const EUROPA_MASS: f64 = 4.799844e22/MASS_DIVISOR;
pub const EUROPA_RADIUS: f64 = 1_560.8;
pub const EUROPA_COLOR: Srgba = BLUE;
pub const EUROPA_NAME: &str = "Europa";

pub const CALLISTO_ORBITAL_RADIUS: f64 = 1_883_000.;
pub const CALLISTO_MASS: f64 = 1.075938e23/MASS_DIVISOR;
pub const CALLISTO_RADIUS: f64 = 2_410.3;
pub const CALLISTO_COLOR: Srgba = YELLOW_100;
pub const CALLISTO_NAME: &str = "Callisto";

pub const GANYMEDE_ORBITAL_RADIUS: f64 = 1_070_000.;
pub const GANYMEDE_MASS: f64 = 1.4819e23/MASS_DIVISOR;
pub const GANYMEDE_RADIUS: f64 = 2_634.1;
pub const GANYMEDE_COLOR: Srgba = GRAY;
pub const GANYMEDE_NAME: &str = "Ganymede";

pub fn jupiter_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(JUPITER_SYSTEM_RADIUS)
        .with_time_step(JUPITER_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, JUPITER_MASS, JUPITER_RADIUS, JUPITER_COLOR.into(), JUPITER_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: IO_ORBITAL_RADIUS, speed: get_orbital_speed(JUPITER_MASS, IO_ORBITAL_RADIUS), start_angle: 0. }, IO_MASS, IO_RADIUS, IO_COLOR.into(), IO_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: EUROPA_ORBITAL_RADIUS, speed: get_orbital_speed(JUPITER_MASS, EUROPA_ORBITAL_RADIUS), start_angle: 0. }, EUROPA_MASS, EUROPA_RADIUS, EUROPA_COLOR.into(), EUROPA_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: CALLISTO_ORBITAL_RADIUS, speed: get_orbital_speed(JUPITER_MASS, CALLISTO_ORBITAL_RADIUS), start_angle: 0. }, CALLISTO_MASS, CALLISTO_RADIUS, CALLISTO_COLOR.into(), CALLISTO_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: GANYMEDE_ORBITAL_RADIUS, speed: get_orbital_speed(JUPITER_MASS, GANYMEDE_ORBITAL_RADIUS), start_angle: 0. }, GANYMEDE_MASS, GANYMEDE_RADIUS, GANYMEDE_COLOR.into(), GANYMEDE_NAME.into()),
        ])
}
//////////////////////////////////////////////////////////

// SATURN SYSTEM /////////////////////////////////////////
pub const SATURN_ORBITAL_RADIUS: f64 = 1.4454e9;

pub const SATURN_SYSTEM_RADIUS: f64 = 6e6;
pub const SATURN_SYSTEM_TIME_STEP: u64 = 1;

pub const SATURN_MASS: f64 = 5.68319e26/MASS_DIVISOR;
pub const SATURN_RADIUS: f64 = 58_232.;
pub const SATURN_COLOR: Srgba = LIGHT_GOLDENROD_YELLOW;
pub const SATURN_NAME: &str = "Saturn";

pub const MIMAS_ORBITAL_RADIUS: f64 = 185_539.;
pub const MIMAS_MASS: f64 = 4e19/MASS_DIVISOR;
pub const MIMAS_RADIUS: f64 = 198.;
pub const MIMAS_COLOR: Srgba = DARK_GRAY;
pub const MIMAS_NAME: &str = "Mimas";

pub const ENCELADUS_ORBITAL_RADIUS: f64 = 237_948.;
pub const ENCELADUS_MASS: f64 = 1.1e20/MASS_DIVISOR;
pub const ENCELADUS_RADIUS: f64 = 252.;
pub const ENCELADUS_COLOR: Srgba = LIGHT_CORAL;
pub const ENCELADUS_NAME: &str = "Enceladus";

pub const TETHYS_ORBITAL_RADIUS: f64 = 294_619.;
pub const TETHYS_MASS: f64 = 6.2e20/MASS_DIVISOR;
pub const TETHYS_RADIUS: f64 = 531.;
pub const TETHYS_COLOR: Srgba = LIGHT_SKY_BLUE;
pub const TETHYS_NAME: &str = "Tethys";

pub const DIONE_ORBITAL_RADIUS: f64 = 377_396.;
pub const DIONE_MASS: f64 = 1.1e21/MASS_DIVISOR;
pub const DIONE_RADIUS: f64 = 561.5;
pub const DIONE_COLOR: Srgba = LIGHT_GRAY;
pub const DIONE_NAME: &str = "Dione";

pub const RHEA_ORBITAL_RADIUS: f64 = 527_108.;
pub const RHEA_MASS: f64 = 2.3e21/MASS_DIVISOR;
pub const RHEA_RADIUS: f64 = 763.5;
pub const RHEA_COLOR: Srgba = GRAY;
pub const RHEA_NAME: &str = "Rhea";

pub const TITAN_ORBITAL_RADIUS: f64 = 1_221_870.;
pub const TITAN_MASS: f64 = 1.35e23/MASS_DIVISOR;
pub const TITAN_RADIUS: f64 = 2_574.5;
pub const TITAN_COLOR: Srgba = YELLOW_50;
pub const TITAN_NAME: &str = "Titan";

pub const LAPETUS_ORBITAL_RADIUS: f64 = 3_560_820.;
pub const LAPETUS_MASS: f64 = 1.8e21/MASS_DIVISOR;
pub const LAPETUS_RADIUS: f64 = 735.;
pub const LAPETUS_COLOR: Srgba = DARK_GREEN;
pub const LAPETUS_NAME: &str = "Lapetus";

pub fn saturn_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(SATURN_SYSTEM_RADIUS)
        .with_time_step(SATURN_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, SATURN_MASS, SATURN_RADIUS, SATURN_COLOR.into(), SATURN_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: MIMAS_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MASS, MIMAS_ORBITAL_RADIUS), start_angle: 0. }, MIMAS_MASS, MIMAS_RADIUS, MIMAS_COLOR.into(), MIMAS_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: ENCELADUS_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MASS, ENCELADUS_ORBITAL_RADIUS), start_angle: 0. }, ENCELADUS_MASS, ENCELADUS_RADIUS, ENCELADUS_COLOR.into(), ENCELADUS_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: TETHYS_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MASS, TETHYS_ORBITAL_RADIUS), start_angle: 0. }, TETHYS_MASS, TETHYS_RADIUS, TETHYS_COLOR.into(), TETHYS_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: DIONE_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MASS, DIONE_ORBITAL_RADIUS), start_angle: 0. }, DIONE_MASS, DIONE_RADIUS, DIONE_COLOR.into(), DIONE_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: RHEA_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MASS, RHEA_ORBITAL_RADIUS), start_angle: 0. }, RHEA_MASS, RHEA_RADIUS, RHEA_COLOR.into(), RHEA_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: TITAN_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MASS, TITAN_ORBITAL_RADIUS), start_angle: 0. }, TITAN_MASS, TITAN_RADIUS, TITAN_COLOR.into(), TITAN_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: LAPETUS_ORBITAL_RADIUS, speed: get_orbital_speed(SATURN_MASS, LAPETUS_ORBITAL_RADIUS), start_angle: 0. }, LAPETUS_MASS, LAPETUS_RADIUS, LAPETUS_COLOR.into(), LAPETUS_NAME.into()),
        ])
}
//////////////////////////////////////////////////////////

// URANUS SYSTEM /////////////////////////////////////////
pub const URANUS_ORBITAL_RADIUS: f64 = 2.9272e9;

pub const URANUS_SYSTEM_RADIUS: f64 = 6e6;
pub const URANUS_SYSTEM_TIME_STEP: u64 = 1;

pub const URANUS_MASS: f64 = 8.618e25/MASS_DIVISOR;
pub const URANUS_RADIUS: f64 = 25_362.;
pub const URANUS_COLOR: Srgba = CORNFLOWER_BLUE;
pub const URANUS_NAME: &str = "Uranus";

pub const MIRANDA_ORBITAL_RADIUS: f64 = 129_390.;
pub const MIRANDA_MASS: f64 = 6293e16/MASS_DIVISOR;
pub const MIRANDA_RADIUS: f64 = 235.8;
pub const MIRANDA_COLOR: Srgba = LIGHT_GRAY;
pub const MIRANDA_NAME: &str = "Miranda";

pub const ARIEL_ORBITAL_RADIUS: f64 = 191_020.;
pub const ARIEL_MASS: f64 = 123_310e16/MASS_DIVISOR;
pub const ARIEL_RADIUS: f64 = 578.9;
pub const ARIEL_COLOR: Srgba = GRAY;
pub const ARIEL_NAME: &str = "Ariel";

pub const UMBRIEL_ORBITAL_RADIUS: f64 = 266_000.;
pub const UMBRIEL_MASS: f64 = 128_850e16/MASS_DIVISOR;
pub const UMBRIEL_RADIUS: f64 = 584.7;
pub const UMBRIEL_COLOR: Srgba = DARK_GRAY;
pub const UMBRIEL_NAME: &str = "Umbriel";

pub const TITANIA_ORBITAL_RADIUS: f64 = 436_300.;
pub const TITANIA_MASS: f64 = 345_500e16/MASS_DIVISOR;
pub const TITANIA_RADIUS: f64 = 788.4;
pub const TITANIA_COLOR: Srgba = LIGHT_SALMON;
pub const TITANIA_NAME: &str = "Titania";

pub const OBERON_ORBITAL_RADIUS: f64 = 583_500.;
pub const OBERON_MASS: f64 = 311_040e16/MASS_DIVISOR;
pub const OBERON_RADIUS: f64 = 761.4;
pub const OBERON_COLOR: Srgba = KHAKI;
pub const OBERON_NAME: &str = "Oberon";

pub fn uranus_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(URANUS_SYSTEM_RADIUS)
        .with_time_step(URANUS_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, URANUS_MASS, URANUS_RADIUS, URANUS_COLOR.into(), URANUS_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: MIRANDA_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MASS, MIRANDA_ORBITAL_RADIUS), start_angle: 0. }, MIRANDA_MASS, MIRANDA_RADIUS, MIRANDA_COLOR.into(), MIRANDA_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: ARIEL_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MASS, ARIEL_ORBITAL_RADIUS), start_angle: 0. }, ARIEL_MASS, ARIEL_RADIUS, ARIEL_COLOR.into(), ARIEL_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: UMBRIEL_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MASS, UMBRIEL_ORBITAL_RADIUS), start_angle: 0. }, UMBRIEL_MASS, UMBRIEL_RADIUS, UMBRIEL_COLOR.into(), UMBRIEL_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: TITANIA_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MASS, TITANIA_ORBITAL_RADIUS), start_angle: 0. }, TITANIA_MASS, TITANIA_RADIUS, TITANIA_COLOR.into(), TITANIA_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: OBERON_ORBITAL_RADIUS, speed: get_orbital_speed(URANUS_MASS, OBERON_ORBITAL_RADIUS), start_angle: 0. }, OBERON_MASS, OBERON_RADIUS, OBERON_COLOR.into(), OBERON_NAME.into()),
        ])
}
//////////////////////////////////////////////////////////

// NEPTUNE SYSTEM /////////////////////////////////////////
pub const NEPTUNE_ORBITAL_RADIUS: f64 = 4.4718e9;

pub const NEPTUNE_SYSTEM_RADIUS: f64 = 6e6;
pub const NEPTUNE_SYSTEM_TIME_STEP: u64 = 1;

pub const NEPTUNE_MASS: f64 = 1.0241e26/MASS_DIVISOR;
pub const NEPTUNE_RADIUS: f64 = 24_622.;
pub const NEPTUNE_COLOR: Srgba = BLUE;
pub const NEPTUNE_NAME: &str = "Neptune";

pub const DESPINA_ORBITAL_RADIUS: f64 = 52_526.;
pub const DESPINA_MASS: f64 = 170e16/MASS_DIVISOR;
pub const DESPINA_RADIUS: f64 = 78.;
pub const DESPINA_COLOR: Srgba = LIGHT_GRAY;
pub const DESPINA_NAME: &str = "Despina";

pub const GALATEA_ORBITAL_RADIUS: f64 = 61_953.;
pub const GALATEA_MASS: f64 = 280e16/MASS_DIVISOR;
pub const GALATEA_RADIUS: f64 = 140.;
pub const GALATEA_COLOR: Srgba = LIGHT_GRAY;
pub const GALATEA_NAME: &str = "Galatea";

pub const LARISSA_ORBITAL_RADIUS: f64 = 73_548.;
pub const LARISSA_MASS: f64 = 380e16/MASS_DIVISOR;
pub const LARISSA_RADIUS: f64 = 190.;
pub const LARISSA_COLOR: Srgba = LIGHT_GRAY;
pub const LARISSA_NAME: &str = "Larissa";

pub const PROTEUS_ORBITAL_RADIUS: f64 = 117_647.;
pub const PROTEUS_MASS: f64 = 3_900e16/MASS_DIVISOR;
pub const PROTEUS_RADIUS: f64 = 210.;
pub const PROTEUS_COLOR: Srgba = LIGHT_GRAY;
pub const PROTEUS_NAME: &str = "Proteus";

pub const TRITON_ORBITAL_RADIUS: f64 = 354_759.;
pub const TRITON_MASS: f64 = 2_139_000e16/MASS_DIVISOR;
pub const TRITON_RADIUS: f64 = 1_352.6;
pub const TRITON_COLOR: Srgba = GRAY;
pub const TRITON_NAME: &str = "Triton";

pub const NEREID_ORBITAL_RADIUS: f64 = 5_504_000.;
pub const NEREID_MASS: f64 = 2400e16/MASS_DIVISOR;
pub const NEREID_RADIUS: f64 = 178.5;
pub const NEREID_COLOR: Srgba = LIGHT_GRAY;
pub const NEREID_NAME: &str = "Nereid";

pub fn neptune_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(NEPTUNE_SYSTEM_RADIUS)
        .with_time_step(NEPTUNE_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, NEPTUNE_MASS, NEPTUNE_RADIUS, NEPTUNE_COLOR.into(), NEPTUNE_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: DESPINA_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MASS, DESPINA_ORBITAL_RADIUS), start_angle: 0. }, DESPINA_MASS, DESPINA_RADIUS, DESPINA_COLOR.into(), DESPINA_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: GALATEA_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MASS, GALATEA_ORBITAL_RADIUS), start_angle: 0. }, GALATEA_MASS, GALATEA_RADIUS, GALATEA_COLOR.into(), GALATEA_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: LARISSA_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MASS, LARISSA_ORBITAL_RADIUS), start_angle: 0. }, LARISSA_MASS, LARISSA_RADIUS, LARISSA_COLOR.into(), LARISSA_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: PROTEUS_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MASS, PROTEUS_ORBITAL_RADIUS), start_angle: 0. }, PROTEUS_MASS, PROTEUS_RADIUS, PROTEUS_COLOR.into(), PROTEUS_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: TRITON_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MASS, TRITON_ORBITAL_RADIUS), start_angle: 0. }, TRITON_MASS, TRITON_RADIUS, TRITON_COLOR.into(), TRITON_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: NEREID_ORBITAL_RADIUS, speed: get_orbital_speed(NEPTUNE_MASS, NEREID_ORBITAL_RADIUS), start_angle: 0. }, NEREID_MASS, NEREID_RADIUS, NEREID_COLOR.into(), NEREID_NAME.into()),
        ])
}
//////////////////////////////////////////////////////////

// PLUTO SYSTEM //////////////////////////////////////
pub const PLUTO_ORBITAL_RADIUS: f64 = 5.9e9;

pub const PLUTO_SYSTEM_RADIUS: f64 = 1e5;
pub const PLUTO_SYSTEM_TIME_STEP: u64 = 1;

pub const PLUTO_MASS: f64 = 1.3e22/MASS_DIVISOR;
pub const PLUTO_RADIUS: f64 = 1_185.;
pub const PLUTO_COLOR: Srgba = LIGHT_SLATE_GREY;
pub const PLUTO_NAME: &str = "Pluto";

pub const CHARON_ORBITAL_RADIUS: f64 = 19_640.;
pub const CHARON_MASS: f64 = 158.7e19/MASS_DIVISOR;
pub const CHARON_RADIUS: f64 = 606.;
pub const CHARON_COLOR: Srgba = LIGHT_GRAY;
pub const CHARON_NAME: &str = "Charon";

pub fn pluto_system() -> GravitySystemBuilder {
    GravitySystemBuilder::new()
        .with_position(StaticPosition::Still)
        .with_radius(PLUTO_SYSTEM_RADIUS)
        .with_time_step(PLUTO_SYSTEM_TIME_STEP)
        .with_static_bodies(&[
            StaticBody::new(StaticPosition::Still, PLUTO_MASS, PLUTO_RADIUS, PLUTO_COLOR.into(), PLUTO_NAME.into()),
            StaticBody::new(StaticPosition::Circular { radius: CHARON_ORBITAL_RADIUS, speed: get_orbital_speed(CHARON_MASS, CHARON_ORBITAL_RADIUS), start_angle: 0. }, CHARON_MASS, CHARON_RADIUS, CHARON_COLOR.into(), CHARON_NAME.into()),
        ])
}
////////////////////////////////////////////////////////