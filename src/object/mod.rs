use bevy::prelude::*;

use self::{spawn::{SpawnObjectEvent, spawn_objects}, physics_future::{PhysicsStateChangeEvent, refresh_physics, PhysicsFuture, update_object_position, UpdatePhysics}, select::{on_select, ObjectsSelectedEvent}, drag::{drag_object, ObjectDraggedEvent}, velocity_arrow::{SpawnVelocityArrowEvent, spawn_velocity_arrow, update_velocity_arrow}};

pub mod object;
pub mod object_bundle;
pub mod drag;
pub mod spawn;
pub mod physics_future;
pub mod select;
pub mod velocity_arrow;

pub struct MassiveObjectPlugin;
impl Plugin for MassiveObjectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObjectResources::default())
           .insert_resource(PhysicsFuture::default())
           .insert_resource(UpdatePhysics(false))
           .add_event::<SpawnObjectEvent>()
           .add_event::<PhysicsStateChangeEvent>()
           .add_event::<ObjectsSelectedEvent>()
           .add_event::<ObjectDraggedEvent>()
           .add_event::<SpawnVelocityArrowEvent>()
           .add_systems(Startup, init)
           .add_systems(Update, (on_select, spawn_velocity_arrow, spawn_objects, drag_object, update_velocity_arrow))
           .add_systems(PostUpdate, (refresh_physics.before(update_object_position), update_object_position));
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

