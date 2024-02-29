use bevy::prelude::*;
use crate::map::Map;
use crate::Params;

pub const NBR : usize = 10_000;
pub const SIZE : f32 = 100.0;
pub const VIEW_DIST : f32 = 1.0;
pub const BORDER : f32 = 1.0;

#[derive(Bundle, Default)]
pub struct BoidBundle {
    mesh: SpriteBundle,
    pos: Pos,
    vel: Vel,
    dir_vec: DirVec,
    prox_cache: ProxCache
}

impl BoidBundle {
    pub fn new(
	pos: Vec2, vel: Vec2,
	mesh: SpriteBundle
    ) -> Self {
	BoidBundle {
	    mesh,
	    pos: Pos(pos),
	    vel: Vel(vel),
	    ..default()
	}
    }
}

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
	app.insert_resource(Map::new(SIZE as usize));
	app.add_systems((
	    boids_proxcache,
	    boids_sep,
	    boids_ali,
	    boids_coh,
	    boids_dir,
	    boids_move,
	    mesh_update
	));
    }
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Pos(pub Vec2);

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Vel(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct DirVec(Vec<Vec2>);

#[derive(Component, Default, Debug)]
pub struct ProxCache(Vec<(Entity,Vec2,f32)>);

fn boids_proxcache(
    map: Res<Map>,
    mut query0: Query<(Entity, &Pos, &mut ProxCache)>,
    query1: Query<&Pos>
) {
    query0.par_iter_mut()
        .for_each_mut(|(e0, Pos(p0), mut prox_cache)| {
	    prox_cache.0.clear();
	    for e1 in map.get(p0, VIEW_DIST) {
		if e0 == e1 { continue }
		let Ok(Pos(p1)) = query1.get(e1) else {
		    error!("entity not found! should never occure!");
		    continue;
		};
		let diff = *p0 - *p1;
		let d = diff.length_squared();
		if d > VIEW_DIST*VIEW_DIST || d == 0.0 { continue }
		prox_cache.0.push((e1, diff, d));
	    }
	});
}

fn boids_sep(
    params: Res<Params>,
    mut query: Query<(&Pos, &ProxCache, &mut DirVec)>,
) {
    for (Pos(p0), ProxCache(prox_cache), mut dir_vec) in &mut query {
	let mut sep = Vec2::default();
	
	for (_,diff,d) in prox_cache {
	    sep += *diff / *d;
	}
	dir_vec.0.push(sep * params.sep);
	dir_vec.0.push(border_vel(p0) * 0.2);	
    }
}

fn boids_coh(
    params: Res<Params>,
    mut query: Query<(&ProxCache, &mut DirVec)>,
) {
    for (ProxCache(prox_cache), mut dir_vec) in &mut query {    
	let mut coh = Vec2::default();

	for (_,diff,_) in prox_cache {
	    coh -= *diff;
	}
	dir_vec.0.push(coh * params.coh);
    }
}

fn boids_ali(
    params: Res<Params>,
    mut query: Query<(&ProxCache, &mut DirVec)>,
    query2: Query<&Vel>
) {
    for (ProxCache(prox_cache), mut dir_vec) in &mut query { 
	let mut ali = Vec2::default();
	
	for (e1,_,_) in prox_cache {
	    let Ok(Vel(v1)) = query2.get(*e1) else {
		error!("entity not found! should never occure!");
		continue;
	    };	    
	    ali += *v1;
	}
	dir_vec.0.push(ali * params.ali);    
    }
}

fn border_vel(p0: &Vec2) -> Vec2 {
    Vec2::new(
	match p0.x {
	    _ if p0.x < -SIZE+BORDER => 1.0,
	    _ if p0.x > SIZE-BORDER => -1.0,
	    _ => 0.0
	},
	match p0.y {
	    _ if p0.y < -SIZE+BORDER => 1.0,
	    _ if p0.y > SIZE-BORDER => -1.0,
	    _ => 0.0
	}
    )
}

fn boids_dir(mut query: Query<(&mut DirVec, &mut Vel)>) {
    for (mut dir_vec, mut vel) in &mut query {
	for v in &dir_vec.0 {
	    vel.0 += *v;
	}
	vel.0 = vel.0.normalize();
	dir_vec.0.clear();
    }
}

fn boids_move(
    time: Res<Time>,
    params: Res<Params>,
    mut map: ResMut<Map>,
    mut query: Query<(Entity, &mut Pos, &Vel)>
) {
    let dt = time.delta_seconds();
    for (e, mut pos, vel) in &mut query {
	let new_pos = pos.0 + vel.0 * dt * params.speed;
	let new_pos = Vec2::new(
	    new_pos.x.clamp(-SIZE, SIZE),
	    new_pos.y.clamp(-SIZE, SIZE)
	);
	if pos.0 != new_pos {
	    map.update(&pos.0, &new_pos, &e);
	}
	pos.0 = new_pos;
    }
}

fn mesh_update(mut query: Query<(&mut Transform, &Pos, &Vel)>) {
    for (mut transform, Pos(pos), Vel(vel)) in &mut query {
	let pos3 = Vec3::new(pos.x, pos.y, 0.0);
	let angle = Vec2::X.angle_between(*vel);
	
	*transform = Transform::from_translation(pos3)
	    .with_rotation(Quat::from_rotation_z(angle));
    }
}
