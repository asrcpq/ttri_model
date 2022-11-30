use crate::cmodel::{Model, Face};

type V2 = rust_stddep::nalgebra::Vector2<f32>;

pub fn v2p4(v2: V2, z: f32) -> [f32; 4] {
	[v2[0], v2[1], z, 1.0]
}

pub struct Pen {
	pub width: f32,
	pub z: f32,
	pub color: [f32; 4],
	pub tex_layer: i32,
}

impl Default for Pen {
	fn default() -> Self {
		Self {
			width: 0f32,
			z: 0f32,
			color: [1f32; 4],
			tex_layer: -2,
		}
	}
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
		m.faces.push(Face::solid(
			[len + 1, len + 2, len],
			color,
		));
		m.faces.push(Face::solid(
			[len + 1, len + 2, len + 3],
			color,
		));
	}

	pub fn draw_rect(&self, m: &mut Model, lu: V2, rd: V2) {
		let z = self.z;
		let color = self.color;
		let ru = V2::new(rd[0], lu[1]);
		let ld = V2::new(lu[0], rd[1]);

		let len = m.vs.len();
		m.vs.extend(vec![
			v2p4(lu, z),
			v2p4(ru, z),
			v2p4(ld, z),
			v2p4(rd, z),
		]);
		m.faces.push(Face::solid(
			[len + 1, len + 2, len],
			color,
		));
		m.faces.push(Face::solid(
			[len + 1, len + 2, len + 3],
			color,
		));
	}

	pub fn draw_line(&self, m: &mut Model, vs: [V2; 2]) {
		let width = self.width;
		let z = self.z;
		let color = self.color;

		let vnorm = (vs[0] - vs[1]).normalize();
		let dp = V2::new(-vnorm[1], vnorm[0]) * width;
		let vlen = m.vs.len();
		m.vs.extend(vec![
			v2p4(vs[0] - dp, z),
			v2p4(vs[0] + dp, z),
			v2p4(vs[1] - dp, z),
			v2p4(vs[1] + dp, z),
		]);
		m.faces.push(Face::solid(
			[vlen, vlen + 1, vlen + 2],
			color,
		));
		m.faces.push(Face::solid(
			[vlen + 3, vlen + 1, vlen + 2],
			color,
		));
	}
}
