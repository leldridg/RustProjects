#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    // the id of the entity this vertex belongs to
    pub entity_id: u32,
    // the initial position of the vertex on the XYZ plane
    pub position: [f32 ; 3],
}

impl Vertex {
    pub fn from_pos(x: f32, y: f32, z: f32) -> Self {
        Self {
            entity_id: 0,
            position: [x, y, z],
        }
    }
}