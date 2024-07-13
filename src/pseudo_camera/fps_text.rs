use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, text::TextLayoutInfo};

use crate::pseudo_camera::camera::CameraState;

#[derive(Component)]
pub struct DebugDisplayText;

#[derive(Component)]
pub struct MeterStickText;

pub fn spawn_debug_text(mut commands: Commands, ) {
    let text_style = TextStyle {
        font_size: 15.,
        ..default()
    };

    commands.spawn((TextBundle::from_sections([
        TextSection::new("FPS:", text_style.clone()),
        TextSection::from_style(text_style.clone()),
        TextSection::new("  CURSOR_POS:", text_style.clone()),
        TextSection::from_style(text_style.clone()),
        TextSection::new("  SCALE:", text_style.clone()),
        TextSection::from_style(text_style.clone()),
    ]), DebugDisplayText));

    commands.spawn((
        TextBundle::from_section("", text_style.clone()).with_style(Style { position_type: PositionType::Absolute, top: Val::Px(5.), right: Val::Px(100.), ..default() }),
        MeterStickText
    ));
}

pub fn update_debug_text(
    mut text_query: Query<&mut Text, With<DebugDisplayText>>,
    mut meter_stick_query: Query<&mut Text, (With<MeterStickText>, Without<DebugDisplayText>)>,
    diagnostics: Res<DiagnosticsStore>,
    window_query: Query<&Window>,
    camera_query: Query<(&CameraState, &Camera, &GlobalTransform)>,
) {
    let mut text = text_query.single_mut();
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed()) {
        text.sections[1].value = format!("{fps:.2}");
    }
    
    let Some(cursor_pos) = window_query.single().cursor_position() else { return };
    let (camera_state, camera, gtrans) = camera_query.single();
    let Some(world_pos) = camera_state.viewport_to_physics_pos(cursor_pos, camera, gtrans).map(|p| camera_state.physics_to_world_pos(p)) else { return };
    text.sections[3].value = format!("{:.4}, {:.4}", world_pos.x, world_pos.y);

    text.sections[5].value = format!("{}", camera_state.get_scale());

    let mut text = meter_stick_query.single_mut();
    let scalar = 10.0f32.powf(-camera_state.get_scale().log10().ceil() + 2.);
    text.sections[0].value = format!("{scalar}");
    
}