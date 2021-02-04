use rmu::vector::{Vector3,Vector2};

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub vertex_normals: Vec<Vector3>,
    pub uv: Vec<Vector2>,
    pub edges: Vec<[u32;2]>,
    /// x is vertex index, y is normal index, z is uv index 
    /// index is start from 1, if index is 0 , it' mean no attribute
    pub faces: Vec<Vec<[u32;3]>>,
}

impl Mesh {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.append(&mut b"mesh data".to_vec());
        result.push(b'\n');

        for vertex in self.vertices.iter() {
            result.push(b'v');
            append_vector3(&mut result, vertex);
            result.push(b'\n');
        }

        for normal in self.vertex_normals.iter() {
            result.push(b'n');
            append_vector3(&mut result, normal);
            result.push(b'\n');
        }

        for uv in self.uv.iter() {
            // c: Coordination
            result.push(b'c');
            append_f32(&mut result, uv.x);
            append_f32(&mut result, uv.y);
            result.push(b'\n');
        }

        for edge in self.edges.iter() {
            result.push(b'e');
            append_u32(&mut result, edge[0]);
            append_u32(&mut result, edge[1]);
            result.push(b'\n');
        }

        for face in self.faces.iter() {
            result.push(b'f');
            for face_attr in face {
                append_u32_3(&mut result, face_attr);
            }
            result.push(b'\n');
        }

        result
    }

    pub fn from_bytes(data: &Vec<u8>) -> Option<Mesh> {
        // TODO: byte to mesh 
        None
    }
}

fn append_vector3(vec: &mut Vec<u8>, vector3: &Vector3) {
    append_f32(vec, vector3.x);
    append_f32(vec, vector3.y);
    append_f32(vec, vector3.z);
}

fn append_u32_3(vec: &mut Vec<u8>, value: &[u32;3]) {
    append_u32(vec, value[0]);
    append_u32(vec, value[1]);
    append_u32(vec, value[2]);
}

fn append_f32(vec: &mut Vec<u8>, value: f32) {
    let v = value.to_le_bytes();
    vec.push(v[0]);
    vec.push(v[1]);
    vec.push(v[2]);
    vec.push(v[3]);
}

fn append_u32(vec: &mut Vec<u8>, value: u32) {
    let v = value.to_le_bytes();
    vec.push(v[0]);
    vec.push(v[1]);
    vec.push(v[2]);
    vec.push(v[3]);
}