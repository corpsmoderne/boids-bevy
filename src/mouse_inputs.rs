use bevy::prelude::*;
//use bevy::input::mouse::{MouseButtonInput,MouseWheel,MouseMotion};
use bevy::input::mouse::{MouseWheel,MouseMotion};
//use bevy::input::ButtonState;

use crate::CameraState;

pub fn mouse_move_events(
    buttons: Res<Input<MouseButton>>,    
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,        
    mut query: Query<(&mut Transform, &mut CameraState)>
) {
    let (mut transform, mut state) = query.single_mut();
    
    for event in mouse_wheel_events.iter() {
	let scale = state.scale;
	state.scale = (scale + event.y * scale)
	    .clamp(0.001, 0.1);
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

/*
pub fn mouse_button_events(
    mut commands: Commands,
    mut mouse_button_events: EventReader<MouseButtonInput>
) {
    for _event in mouse_button_events.iter() {
	
	if event.button == MouseButton::Left &&
	    event.state == ButtonState::Pressed {
	    } else if event.button == MouseButton::Right &&
	    event.state == ButtonState::Pressed {
		for (entity,_) in &query {
		    commands.entity(entity).despawn();
		    terrain_events
			.send(GenTerrainEvent { seed: rand::random() });
		}
	    }
	 
    }
}
*/
