use std::{
	ops::Add,
	sync::{Arc, RwLock},
};

use anyhow::Result;
use macroquad::prelude::*;
use rayon::prelude::*;

pub struct DrawPlan(pub Vec<DrawCall>);

pub type Color = (u8, u8, u8);

/// A planned draw call. Parsed from the Lua script.
pub enum DrawCall {
	/// Reset everything.
	Home,
	/// Move the pen by the given vector.
	Move(Vec2),
	/// Scale the coordinate system.
	Scale(Vec2),
	/// Rotate the coordinate system.
	Rotate(f32),
	/// Set the pen's color.
	Color(Color),
	/// Set the pen's opacity.
	Power(u8),
	/// Turn the pen on or off.
	Toggle(bool),
}

impl TryFrom<&str> for DrawCall {
	type Error = anyhow::Error;
	fn try_from(s: &str) -> Result<Self> {
		let words = s.split_whitespace().collect::<Vec<&str>>();
		Ok(match words[0] {
			"home" => DrawCall::Home,
			"move" => DrawCall::Move(Vec2::new(
				words[1].parse().unwrap(),
				words[2].parse().unwrap(),
			)),
			"scale" => DrawCall::Scale(Vec2::new(
				words[1].parse().unwrap(),
				words[2].parse().unwrap(),
			)),
			"rotate" => DrawCall::Rotate(words[1].parse().unwrap()),
			"color" => DrawCall::Color((
				words[1].parse().unwrap(),
				words[2].parse().unwrap(),
				words[2].parse().unwrap(),
			)),
			"power" => DrawCall::Power(words[1].parse().unwrap()),
			"toggle" => DrawCall::Toggle(words[1].parse().unwrap()),
			_ => panic!("Unknown draw call: {}", s),
		})
	}
}

impl From<Vec<String>> for DrawPlan {
	fn from(s: Vec<String>) -> Self {
		DrawPlan(
			s.iter()
				.filter(|s| !s.is_empty())
				.map(|s| s.as_str().try_into())
				.flatten()
				.collect::<Vec<DrawCall>>(),
		)
	}
}

impl Add<DrawPlan> for DrawPlan {
	type Output = DrawPlan;
	fn add(self, other: DrawPlan) -> Self::Output {
		DrawPlan(self.0.into_iter().chain(other.0).collect())
	}
}

impl Add<DrawCall> for DrawPlan {
	type Output = DrawPlan;
	fn add(self, other: DrawCall) -> Self::Output {
		DrawPlan(self.0.into_iter().chain(vec![other]).collect())
	}
}

impl Add<DrawCall> for DrawCall {
	type Output = DrawPlan;
	fn add(self, other: DrawCall) -> Self::Output {
		DrawPlan(vec![self, other])
	}
}

pub trait Draw {
	fn draw(&self) -> ();
}

lazy_static::lazy_static! {
	pub static ref X_AXIS: Arc<RwLock<Vec2>> = Arc::new(RwLock::new(Vec2::X));
	pub static ref Y_AXIS: Arc<RwLock<Vec2>> = Arc::new(RwLock::new(Vec2::Y));
	pub static ref POS: Arc<RwLock<Vec2>> = Arc::new(RwLock::new(Vec2::ZERO));
  pub static ref COLOR: Arc<RwLock<Color>> = Arc::new(RwLock::new((255, 255, 255)));
  pub static ref POWER: Arc<RwLock<u8>> = Arc::new(RwLock::new(255));
  pub static ref TURNED_ON: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
}

impl Draw for DrawPlan {
	fn draw(&self) -> () {
		for call in &self.0 {
			call.draw();
		}
	}
}

impl Draw for DrawCall {
	fn draw(&self) -> () {
		match self {
			DrawCall::Home => {
				*X_AXIS.write().unwrap() = Vec2::X;
				*Y_AXIS.write().unwrap() = Vec2::Y;
				*POS.write().unwrap() = Vec2::ZERO;
				*COLOR.write().unwrap() = (255, 255, 255);
				*POWER.write().unwrap() = 255;
				*TURNED_ON.write().unwrap() = true;
			}
			DrawCall::Move(vector) => {
				let vector =
					(*X_AXIS.read().unwrap() * vector.x) + (*Y_AXIS.read().unwrap() * vector.y);
				let old_pos = POS.read().unwrap().clone();
				*POS.write().unwrap() += vector;
				let new_pos = POS.read().unwrap().clone();
				// actually draw a line
				let (r, g, b) = *COLOR.read().unwrap();
				let a = *POWER.read().unwrap();
				if *TURNED_ON.read().unwrap() {
					draw_line(
						old_pos.x,
						old_pos.y,
						new_pos.x,
						new_pos.y,
						1.0,
						macroquad::color::Color::from_rgba(r, g, b, a),
					)
				}
			}
			DrawCall::Scale(vector) => {
				*X_AXIS.write().unwrap() *= vector.x;
				*Y_AXIS.write().unwrap() *= vector.y;
			}
			DrawCall::Rotate(degrees) => {
				let radians = degrees.to_radians();
				let sin = radians.sin();
				let cos = radians.cos();
				let old_x = *X_AXIS.read().unwrap();
				let old_y = *Y_AXIS.read().unwrap();
				*X_AXIS.write().unwrap() =
					Vec2::new(cos * old_x.x - sin * old_x.y, sin * old_x.x + cos * old_x.y);
				*Y_AXIS.write().unwrap() =
					Vec2::new(cos * old_y.x - sin * old_y.y, sin * old_y.x + cos * old_y.y);
			}
			DrawCall::Color(c) => {
				*COLOR.write().unwrap() = *c;
			}
			DrawCall::Power(power) => {
				*POWER.write().unwrap() = *power;
			}
			DrawCall::Toggle(on) => {
				*TURNED_ON.write().unwrap() = *on;
			}
		}
	}
}
