use bevy::prelude::*;

use std::collections::HashSet;

#[derive(Resource,Default,Debug)]
pub struct Map { size: usize,
		 map: Vec<Vec<HashSet<Entity>>>
}

impl Map {
    pub fn new(size: usize) -> Self {
	Map { size,
	      map: vec! [ vec![ HashSet::default() ; size*2 +1 ] ; size*2 + 1]
	}
    }

    pub fn insert(&mut self, pos: &Vec2, entity: &Entity) {
	let (x, y) = self.get_key(pos);
	self.map[y][x].insert(*entity);
    }
    
    pub fn update(&mut self, old_pos: &Vec2, new_pos: &Vec2, entity: &Entity) {
	let (old_x, old_y) = self.get_key(old_pos);
	let (new_x, new_y) = self.get_key(new_pos);
	self.map[old_y][old_x].remove(entity);
	self.map[new_y][new_x].insert(*entity);
    }

    pub fn get(&self, pos: &Vec2, view_dist: f32) -> Vec<Entity> {
	let vd = Vec2::new(view_dist, view_dist);
	let (from_x, from_y) = self.get_key(&(*pos - vd));
	let (to_x, to_y) = self.get_key(&(*pos + vd));
	let mut v = vec![];

	for y in from_y..=to_y {
	    for x in from_x..=to_x {
		v.extend(self.map[y][x].iter());
	    }
	}
	v
    }

    fn get_key(&self, pos: &Vec2) -> (usize, usize) {
	let size = self.size as i32;
	((pos.x as i32 + size)
	 .clamp(0, size*2) as usize,
	 (pos.y as i32 + size)
	 .clamp(0, size*2) as usize)
    }
}


/*
use std::collections::{HashMap,HashSet};

#[derive(Resource,Default,Debug)]
pub struct Map(HashMap<(i32,i32), HashSet<Entity>>);

impl Map {
    pub fn update(&mut self, old_pos: &Vec2, new_pos: &Vec2, entity: &Entity) {
	let old_p = (old_pos.x as i32, old_pos.y as i32);
	let new_p = (new_pos.x as i32, new_pos.y as i32);

	self.0.entry(old_p)
	    .and_modify(| set | { set.remove(entity); });
	self.0.entry(new_p)
	    .and_modify(| set | { set.insert(*entity); })
	    .or_insert([*entity].into());
    }

    pub fn get(&self, pos: &Vec2, view_dist: f32) -> Vec<Entity> {
	let from_x = (pos.x - view_dist) as i32;
	let from_y = (pos.y - view_dist) as i32;
	let to_x = (pos.x + view_dist) as i32;
	let to_y = (pos.y + view_dist) as i32;
	let mut v = vec![];

	for y in from_y..=to_y {
	    for x in from_x..=to_x {
		if let Some(set) = self.0.get(&(x,y)) {
		    v.extend(set.iter());
		}
	    }
	}
	v
    }
}
*/
