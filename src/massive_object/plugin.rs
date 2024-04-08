use super::*;

pub struct MassiveObjectPlugin;
impl Plugin for MassiveObjectPlugin {
    fn build(&self, app: &mut App) {
        // insert resource for drawing path prediction?
        app.insert_resource(PhysicsFuture::default())
            .add_event::<ModifyObjectEvent>();
    }
}