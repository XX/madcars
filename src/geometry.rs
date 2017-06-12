use amethyst::renderer::VertexPosNormal;
use cgmath::InnerSpace;
use cgmath::Vector3;
use genmesh::MapToVertices;
use genmesh::Triangulate;
use genmesh::Vertices;
use genmesh::generators::SphereUV;

pub fn gen_sphere(u: usize, v: usize) -> Vec<VertexPosNormal>
{
    let data: Vec<VertexPosNormal> = SphereUV::new(u, v)
        .vertex(|(x, y, z)| {
            VertexPosNormal {
                pos: [x, y, z],
                normal: Vector3::new(x, y, z).normalize().into(),
                tex_coord: [0., 0.],
            }
        })
        .triangulate()
        .vertices()
        .collect();
    data
}