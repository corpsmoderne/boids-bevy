mod map;
mod boids;
mod camera;

use bevy::prelude::*;
use bevy::diagnostic::{ Diagnostics, FrameTimeDiagnosticsPlugin };
use rand::{thread_rng, Rng};
use std::time::Duration;

use boids::{NBR,BORDER,SIZE,BoidPlugin,BoidBundle};
use map::Map;
use camera::{CameraPlugin,CamBundle};

#[derive(Resource,Debug)]
pub struct Params {
    sep: f32,
    ali: f32,
    coh: f32,
    speed: f32
}

impl Default for Params {
    fn default() -> Self {
	Params {
	    sep: 0.02,
	    ali: 0.2,
	    coh: 0.02,
	    speed: 1.0
	}
    }
}

#[derive(Component)]
struct StatsText;

#[derive(Resource)]
struct FpsTimer(Timer);

fn main() {
    App::new()
	.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy :: Boids".into(),
                resolution: (900., 900.).into(),
		..default()
	    }),
	    ..default()
	}))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
	.init_resource::<Params>()
	.add_plugin(BoidPlugin)
	.add_plugin(CameraPlugin)
	.insert_resource(FpsTimer(Timer::new(Duration::from_secs_f32(0.5),
					     TimerMode::Repeating)))
	.add_system(update_stats)
	.add_startup_system(setup)
	.run();
}

fn setup(
    mut commands: Commands,
    mut map: ResMut<Map>,    
    asset_server: Res<AssetServer>
) {
    // Camera
    commands.spawn(CamBundle::new(0.1)); 

    //BG
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bg.png"),
	transform: Transform::from_scale(Vec3::new(SIZE as f32 / 512.0,
						   SIZE as f32 / 512.0, 1.0))
	    .with_translation(Vec3::new(0.0, 0.0, -1.0)),
        ..default()
    });
		   
    let tex = asset_server.load("boid.png");
    
    let mut rng = thread_rng();
    for _ in 0..NBR {
	let pos = Vec2::new(rng.gen_range(-SIZE+BORDER..SIZE-BORDER),
			    rng.gen_range(-SIZE+BORDER..SIZE-BORDER));	    
	let v = Vec2::new(rng.gen_range(-1.0..1.0),
			  rng.gen_range(-1.0..1.0)).normalize();
	let sprite = SpriteBundle {
	    sprite: Sprite {
		custom_size: Some(Vec2::new(2.0, 2.0)),
		..default()
	    },
	    texture: tex.clone(),
            ..default()
	};
	
	let entity = commands.spawn(BoidBundle::new(pos, v, sprite)).id();
	map.insert(&pos, &entity);
    }

    // Unashamely stolen from the website examples...
    let text_section = move |color, value: &str| {
        TextSection::new(
            value,
            TextStyle {
                font: asset_server.load("fonts/DejaVuSans.ttf"),
                font_size: 20.0,
                color,
            },
        )
    };
    
    commands.spawn((
        TextBundle::from_sections([
            text_section(Color::GREEN, "Boid Count: "),
            text_section(Color::CYAN, &format!("{}", NBR)),
            text_section(Color::GREEN, "\nFPS: "),
            text_section(Color::CYAN, ""),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        StatsText,
    ));
    
}

fn update_stats(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<StatsText>>,
    mut fps_timer: ResMut<FpsTimer>,
    time: Res<Time>    
) {
    fps_timer.0.tick(time.delta());
    if !fps_timer.0.finished() {
	return;
    }
    
    let mut text = query.single_mut();    
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
	if let Some(ema) = fps.smoothed() {
	    text.sections[3].value = format!("{ema:.2}");
	}
    }
}
