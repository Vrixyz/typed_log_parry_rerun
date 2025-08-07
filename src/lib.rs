use std::any::TypeId;

use parry3d::{
    math::Real,
    na::Vector3, //transformation::TriangleFacet
};
use rerun::{LineStrip3D, LineStrips3D, Vec3D};
use typed_log::Loggable;
use typed_log_rerun::get_rr;

pub fn register_typed_loggers() {
    // typed_log::push_log_impl(log_triangles_points);
    typed_log::push_log_impl(log_points);
    typed_log::push_log_impl(log_vertices_indices);
}

// pub struct TrianglePoints<'a> {
//     record: Option<&'a str>,
//     triangles: &'a [TriangleFacet],
//     points: &'a [Vector3<Real>],
// }

// impl<'a> Loggable for TrianglePoints<'a> {
//     fn type_id(&self) -> std::any::TypeId {
//         TypeId::of::<TrianglePoints>()
//     }
// }

pub struct Points<'a> {
    pub record: Option<&'a str>,
    pub points: &'a [Vector3<Real>],
}

impl Loggable for Points<'_> {
    fn type_id(&self) -> std::any::TypeId {
        TypeId::of::<Points>()
    }
}

pub struct VerticesIndices<'a> {
    pub record: Option<&'a str>,
    pub vertices: &'a [Vector3<Real>],
    pub indices: &'a [[u32; 3]],
}

impl Loggable for VerticesIndices<'_> {
    fn type_id(&self) -> std::any::TypeId {
        TypeId::of::<VerticesIndices>()
    }
}

// pub fn log_triangles_points(triangle_points: &TrianglePoints) {
//     let TrianglePoints {
//         record,
//         triangles,
//         points,
//     } = triangle_points;
//     for (i, TriangleFacet { pts, .. }) in triangles.iter().enumerate() {
//         let (a, b, c) = (pts[0], pts[1], pts[2]);
//         let (a, b, c) = (points[a], points[b], points[c]);
//         let (a, b, c) = (
//             Vec3D::new(a.x, a.y, a.z),
//             Vec3D::new(b.x, b.y, b.z),
//             Vec3D::new(c.x, c.y, c.z),
//         );
//         let ls = LineStrip3D::from_iter([a, b, c, a].iter());
//         get_rr()
//             .log(
//                 format!("{}/{i}/", record.unwrap_or("triangle facets")),
//                 &LineStrips3D::new([ls]),
//             )
//             .unwrap();
//     }
// }

pub fn log_points(points: &Points) {
    get_rr()
        .log(
            format!("points: {}", points.record.unwrap_or_default()),
            &rerun::Points3D::new(
                points
                    .points
                    .iter()
                    .map(|p| Vec3D::new(p.x, p.y, p.z))
                    .collect::<Vec<_>>(),
            ),
        )
        .unwrap();
}

pub fn log_vertices_indices(vertices_indices: &VerticesIndices) {
    for (i, [a, b, c]) in vertices_indices.indices.iter().enumerate() {
        let (a, b, c) = (
            vertices_indices.vertices[*a as usize],
            vertices_indices.vertices[*b as usize],
            vertices_indices.vertices[*c as usize],
        );
        let (a, b, c) = (
            Vec3D::new(a.x, a.y, a.z),
            Vec3D::new(b.x, b.y, b.z),
            Vec3D::new(c.x, c.y, c.z),
        );
        let ls = LineStrip3D::from_iter([a, b, c, a].iter());
        //triangles.push(ls);

        get_rr()
            .log(
                format!(
                    "{}/{i}/",
                    vertices_indices.record.unwrap_or("triangle facets")
                ),
                &LineStrips3D::new([ls]),
            )
            .unwrap();
    }
}
