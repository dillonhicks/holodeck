use crate::deps::kiss3d::ncollide3d::na::{
    Point2,
    RealField,
    Vector3,
};

use crate::deps::kiss3d::ncollide3d::procedural::{
    utils,
    IndexBuffer,
    TriMesh,
};


/// ripped from nalgebra's procedural generation for a sphere and changed to a shell
/// by reflecting the normal for the component triangles of the sphere mesh.
pub fn unit_shell<N: RealField>(
    ntheta_subdiv: u32,
    nphi_subdiv: u32,
) -> TriMesh<N> {
    use crate::deps::kiss3d::ncollide3d::na;

    let pi = N::pi();
    let two_pi = N::two_pi();
    let pi_two = N::frac_pi_2();
    let duvtheta = N::one() / na::convert(ntheta_subdiv as f64); // step of uv.x coordinates.
    let duvphi = N::one() / na::convert(nphi_subdiv as f64); // step of uv.y coordinates.
    let dtheta = two_pi * duvtheta;
    let dphi = pi * duvphi;

    let mut coords = Vec::new();
    let mut curr_phi = -pi_two;

    for _ in 0..nphi_subdiv + 1 {
        utils::push_circle(
            curr_phi.cos(),
            ntheta_subdiv + 1,
            dtheta,
            curr_phi.sin(),
            &mut coords,
        );
        curr_phi = curr_phi + dphi;
    }

    // the normals are the same as the coords
    let normals: Vec<Vector3<N>> = coords.iter().map(|p| p.coords).collect();

    // index buffer
    let mut idx = Vec::new();

    for i in 0..nphi_subdiv {
        let bottom = i * (ntheta_subdiv + 1);
        let up = bottom + (ntheta_subdiv + 1);
        utils::push_open_ring_indices(up, bottom, ntheta_subdiv + 1, &mut idx);
    }

    let mut uvs = Vec::new();
    let mut curr_uvphi = na::zero::<N>();

    for _ in 0..nphi_subdiv + 1 {
        let mut curr_uvtheta = na::zero::<N>();

        for _ in 0..ntheta_subdiv + 1 {
            uvs.push(Point2::new(curr_uvtheta, curr_uvphi));
            curr_uvtheta = curr_uvtheta + duvtheta;
        }

        curr_uvphi = curr_uvphi + duvphi;
    }

    let mut res = TriMesh::new(coords, Some(normals), Some(uvs), Some(IndexBuffer::Unified(idx)));

    let _0_5: N = na::convert(0.5);
    res.scale_by_scalar(_0_5);

    res
}
