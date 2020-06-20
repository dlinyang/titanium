use super::RenderData;
use crate::base::material::Material;
use crate::base::mesh::Mesh;
use crate::base::Index;
use crate::base::Vertex;
use rmu::raw::Mat4f;

pub trait RenderMesh {
    fn points(mesh: &Mesh, material: &Material, transform: Mat4f) -> Self;
    fn edges(mesh: &Mesh, material: &Material, transform: Mat4f) -> Self;
    fn flat(mesh: &Mesh, material: &Material, transform: Mat4f) -> Self;
    fn smooth(mesh: &Mesh, material: &Material, transform: Mat4f) -> Self;
}

impl RenderMesh for RenderData {
    fn points(mesh: &Mesh, material: &Material, transform: Mat4f) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for vertex in &mesh.vertices {
            vertices.push(Vertex::from(*vertex));
        }

        for face in &mesh.faces {
            for attr in face {
                indices.push(attr[0] - 1);
                let v = attr[0] as usize;
                let n = attr[1] as usize;
                let uv = attr[2] as usize;

                if v != 0 {
                    if n != 0 {
                        vertices[v - 1].normal = mesh.vertex_normals[n - 1].into();
                    }

                    if uv != 0 {
                        vertices[v - 1].tex_coordinate = mesh.uv[uv - 1].into();
                    }
                }
            }
        }

        Self::new(
            vertices,
            Index::Points(indices),
            material.clone(),
            transform,
        )
    }

    fn edges(mesh: &Mesh, material: &Material, transform: Mat4f) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();

        for vertex in &mesh.vertices {
            vertices.push(Vertex::from(*vertex));
        }

        let mut indices: Vec<u32> = Vec::new();

        for face in &mesh.faces {
            let mut edges_indices: Vec<u32> = Vec::new();
            for attr in face {
                edges_indices.push(attr[0] - 1);
                edges_indices.push(attr[0] - 1);

                let v = attr[0] as usize;
                let n = attr[1] as usize;
                let uv = attr[2] as usize;

                if v != 0 {
                    if n != 0 {
                        vertices[v - 1].normal = mesh.vertex_normals[n - 1].into();
                    }

                    if uv != 0 {
                        vertices[v - 1].tex_coordinate = mesh.uv[uv - 1].into();
                    }
                }
            }

            edges_indices.remove(0);
            let i = edges_indices.len() - 1;
            edges_indices.remove(i);

            indices.append(&mut edges_indices);
        }

        Self::new(vertices, Index::EdgeLists(indices), material.clone(), transform)
    }

    fn smooth(mesh: &Mesh, material: &Material, transform: Mat4f) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();

        for vertex in &mesh.vertices {
            vertices.push(Vertex::from(*vertex));
        }

        let mut faces: Vec<Vec<u32>> = Vec::new();

        for face in &mesh.faces {
            let mut face_indices: Vec<u32> = Vec::new();
            for attr in face {
                let v = attr[0] as usize;
                let n = attr[1] as usize;
                let uv = attr[2] as usize;

                if v != 0 {
                    face_indices.push(v as u32);

                    if n != 0 {
                        vertices[v].normal = mesh.vertex_normals[n].into();
                    }

                    if uv != 0 {
                        vertices[v].tex_coordinate = mesh.uv[uv].into();
                    }
                }
            }
            faces.push(face_indices);
        }

        let indices = get_faces_indices(faces);

        Self::new(vertices, Index::TriangleFace(indices), material.clone(), transform)
    }

    fn flat(mesh: &Mesh, material: &Material, transform: Mat4f) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Vec<u32>> = Vec::new();

        let mut index: u32 = 0;
        for face in &mesh.faces {
            let mut face_indices: Vec<u32> = Vec::new();
            for attr in face {
                let v = attr[0] as usize;
                let n = attr[1] as usize;
                let uv = attr[2] as usize;

                if v != 0 {
                    let vertex = mesh.vertices[v - 1].into();
                    let normal = if n != 0 {
                        mesh.vertex_normals[n - 1].into()
                    } else {
                        [0.0, 0.0, 0.0]
                    };
                    let tex_coordinate = if uv != 0 {
                        mesh.uv[uv - 1].into()
                    } else {
                        [0.0, 0.0]
                    };

                    vertices.push(Vertex::new(vertex, normal, tex_coordinate));
                    face_indices.push(index);

                    index = index + 1;
                }
            }
            faces.push(face_indices);
        }

        let indices = get_faces_indices(faces);

        Self::new(vertices, Index::TriangleFace(indices), material.clone(), transform)
    }
}

//polygon faces to triangle list faces
pub fn get_faces_indices(faces: Vec<Vec<u32>>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for face in &faces {
        if face.len() > 2 {
            let first_vertex_index_of_face = face[0];
            let mut i = 1;
            while i < face.len() - 1 {
                result.push(first_vertex_index_of_face);
                result.push(face[i]);
                result.push(face[i + 1]);
                i = i + 1;
            }
        }
    }
    result
}
