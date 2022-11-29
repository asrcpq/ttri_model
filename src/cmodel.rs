#[derive(Clone, Default)]
pub struct Model {
	pub vs: Vec<[f32; 4]>,
	pub uvs: Vec<[f32; 2]>,
	pub faces: Vec<Face>,
}

impl std::fmt::Debug for Model {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{v: {}, uv: {}, f: {}}}",
			self.vs.len(),
			self.uvs.len(),
			self.faces.len(),
		)
	}
}

#[derive(Clone, Debug)]
pub struct Face {
	pub color: [f32; 4],
	pub vid: [usize; 3],
	pub uvid: [usize; 3],
	pub layer: i32,
}

impl Face {
	pub fn solid(vid: [usize; 3], color: [f32; 4]) -> Face {
		Self {
			// -1 is for placeholder in bindless texture (though no real difference)
			layer: -2,
			uvid: [0; 3],
			vid,
			color,
		}
	}
}
