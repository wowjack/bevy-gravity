use bevy::prelude::*;

use self::{spawn::{SpawnObjectEvent, spawn_objects}, physics_future::{PhysicsStateChange, refresh_physics, PhysicsFuture, update_object_position}, select::{on_select, ObjectsSelectedEvent}};

pub mod object;
pub mod object_bundle;
pub mod spawn;
pub mod physics_future;
pub mod select;

pub struct MassiveObjectPlugin;
impl Plugin for MassiveObjectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObjectResources::default())
           .insert_resource(PhysicsFuture::default())
           .add_event::<SpawnObjectEvent>()
           .add_event::<PhysicsStateChange>()
           .add_event::<ObjectsSelectedEvent>()
           .add_systems(Startup, init)
           .add_systems(Update, (spawn_objects, refresh_physics, update_object_position, on_select));
    }
}

#[derive(Resource, Default)]
pub struct ObjectResources {
    circle_mesh: Option<Handle<Mesh>>,
    circle_material: Option<Handle<ColorMaterial>>,
}

fn init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_resources: ResMut<ObjectResources>,
) {
    game_resources.circle_mesh = Some(meshes.add(shape::Circle {radius: 0.5, vertices: 100}.into()).into());
    game_resources.circle_material = Some(materials.add(ColorMaterial::from(Color::PURPLE)));
}

