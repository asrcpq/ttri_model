use crate::cmodel::{Model, Face};

type V2 = rust_stddep::nalgebra::Vector2<f32>;

pub trait Drawable {
	fn draw_dot(&mut self, pos: V2, width: f32, z: f32, color: [f32; 4]);
}

impl Drawable for Model {
	fn draw_dot(&mut self, pos: V2, width: f32, z: f32, color: [f32; 4]) {
		let len = self.vs.len();
		self.vs.extend(vec![
			[pos[0] - width, pos[1] - width, z, 1.0],
			[pos[0] - width, pos[1] + width, z, 1.0],
			[pos[0] + width, pos[1] - width, z, 1.0],
			[pos[0] + width, pos[1] + width, z, 1.0],
		]);
		self.faces.push(Face {
			layer: -2,
			uvid: [0; 3],
			color,
			vid: [len, len + 1, len + 2],
		});
		self.faces.push(Face {
			layer: -2,
			uvid: [0; 3],
			color,
			vid: [len + 1, len + 2, len + 3],
		});
	}
}
