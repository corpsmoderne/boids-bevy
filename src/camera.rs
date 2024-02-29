use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel,MouseMotion};

#[derive(Bundle, Default)]
pub struct CamBundle {
    cam: Camera2dBundle,
    state: CameraState
}

impl CamBundle {
    pub fn new(scale: f32) -> Self {
	CamBundle {
	    cam: Camera2dBundle {
		transform: Transform::from_scale(Vec3::new(scale, scale, 1.0)),
		..default()
	    },
	    state: CameraState { scale, ..default() }
	}
    }
}

#[derive(Component, Default)]
pub struct CameraState {
    scale: f32,
    pos: Vec2
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
	app.add_system(mouse_move_events);
    }
}

fn mouse_move_events(
    buttons: Res<Input<MouseButton>>,    
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,        
    mut query: Query<(&mut Transform, &mut CameraState)>
) {
    let (mut transform, mut state) = query.single_mut();
    
    for event in mouse_wheel_events.iter() {
	let scale = state.scale;
	state.scale = (scale + event.y * 0.25 * scale)
	    .clamp(0.001, 1.0);
	*transform = transform.with_scale(Vec3::new(state.scale,
						    state.scale, 1.0));
    }
    if buttons.pressed(MouseButton::Left) {
	let scale = state.scale;
	for event in mouse_motion_events.iter() {
	    state.pos -= event.delta * scale;
	}
	transform.translation = Vec3::new(state.pos.x, -state.pos.y, 0.0);
    }
}

