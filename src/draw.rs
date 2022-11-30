use crate::cmodel::{Model, Face};

type V2 = rust_stddep::nalgebra::Vector2<f32>;

pub fn v2p4(v2: V2, z: f32) -> [f32; 4] {
	[v2[0], v2[1], z, 1.0]
}

pub struct Pen {
	pub width: f32,
	pub z: f32,
	pub color: [f32; 4],
}

impl Pen {
	pub fn draw_dot(&self, m: &mut Model, pos: V2) {
		let width = self.width;
		let z = self.z;
		let color = self.color;

		let len = m.vs.len();
		m.vs.extend(vec![
			[pos[0] - width, pos[1] - width, z, 1.0],
			[pos[0] - width, pos[1] + width, z, 1.0],
			[pos[0] + width, pos[1] - width, z, 1.0],
			[pos[0] + width, pos[1] + width, z, 1.0],
		]);
		m.faces.push(Face {
			layer: -2,
			uvid: [0; 3],
			color,
			vid: [len, len + 1, len + 2],
		});
		m.faces.push(Face {
			layer: -2,
			uvid: [0; 3],
			color,
			vid: [len + 1, len + 2, len + 3],
		});
	}
}
