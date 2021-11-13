use std::collections::HashMap;

use draw::{DrawCall, DrawPlan};
use macroquad::{color, prelude::*};
use mlua::Lua;
use rayon::{iter::plumbing::UnindexedProducer, prelude::*};

use crate::draw::Draw;
mod draw;

const LUA_PRELUDE: &str = include_str!("prelude.lua");
const ROUTINE_DIR: &str = "routines";

#[macroquad::main("FantaCon")]
async fn main() {
	// Set up Lua
	let lua = Lua::new();
	let globals = lua.globals();
	lua.load(LUA_PRELUDE)
		.exec()
		.expect("Lua prelude failed to execute.");
	// Create the draw instruction table.
	globals
		.set("DRAW", HashMap::<String, Vec<String>>::new())
		.unwrap();
	// Create the draw dirty flag.
	// Anything that draws should set this.
	globals.set("DRAW_DIRTY", false).unwrap();
	// Create the work table.
	globals
		.set("WORK", HashMap::<String, mlua::Value>::new())
		.unwrap();
	// Create the save data table.
	globals
		.set("SAVE", HashMap::<String, mlua::Value>::new())
		.unwrap();
	// Create the save dirty flag.
	// Anything that saves should set this.
	globals.set("SAVE_DIRTY", false).unwrap();
	// Load all of the routines into memory.
	let routines: Vec<String> = std::fs::read_dir("routines")
		.unwrap()
		.into_iter()
		.flatten()
		.map(|f| f.file_name().into_string())
		.flatten()
		.filter(|n| n.ends_with(".lua"))
		.map(|n| std::fs::read_to_string(std::path::Path::new(ROUTINE_DIR).join(n)))
		.flatten()
		.collect();
	let start = std::time::Instant::now();
	clear_background(color::BLACK);
	loop {
		// Write the global timer.
		let now = std::time::Instant::now();
		globals
			.set("TIMER", now.duration_since(start).as_secs_f32())
			.unwrap();
		// Execute the routines.
		for routine in routines.iter() {
			lua.load(routine).exec().unwrap();
		}
		// Check if the draw table is dirty.
		if globals.get("DRAW_DIRTY").unwrap() {
			globals.set("UP", is_key_down(KeyCode::Up)).unwrap();
			globals.set("DOWN", is_key_down(KeyCode::Down)).unwrap();
			globals.set("LEFT", is_key_down(KeyCode::Left)).unwrap();
			globals.set("RIGHT", is_key_down(KeyCode::Right)).unwrap();
			globals.set("Z", is_key_down(KeyCode::Z)).unwrap();
			globals.set("X", is_key_down(KeyCode::X)).unwrap();
			clear_background(color::BLACK);
			// Read in the new draw calls
			let draw_calls: DrawPlan = globals
				.get::<_, HashMap<String, Vec<String>>>("DRAW")
				.unwrap()
				.iter()
				.map(|(_, v)| DrawPlan::from(v.clone()))
				.fold(DrawPlan(vec![]), |acc, v| acc + v);
			// Draw the new draw calls.
			draw_calls.draw();
			// Clear the dirty flag.
			globals.set("DRAW_DIRTY", false).unwrap();
			next_frame().await
		}
	}
}
