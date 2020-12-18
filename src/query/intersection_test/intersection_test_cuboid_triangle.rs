use crate::math::{Isometry, Real};
use crate::query::sat;
use crate::shape::{Cuboid, Triangle};

/// Intersection test between a triangle and a cuboid.
#[inline]
pub fn intersection_test_triangle_cuboid(
    pos12: &Isometry<Real>,
    triangle1: &Triangle,
    cuboid2: &Cuboid,
) -> bool {
    intersection_test_cuboid_triangle(&pos12.inverse(), cuboid2, triangle1)
}

/// Intersection test between a cuboid and a triangle.
#[inline]
pub fn intersection_test_cuboid_triangle(
    pos12: &Isometry<Real>,
    cube1: &Cuboid,
    triangle2: &Triangle,
) -> bool {
    let sep1 =
        sat::cuboid_support_map_find_local_separating_normal_oneway(cube1, triangle2, &pos12).0;
    if sep1 > 0.0 {
        return false;
    }

    let pos21 = pos12.inverse();
    let sep2 = sat::triangle_cuboid_find_local_separating_normal_oneway(triangle2, cube1, &pos21).0;
    if sep2 > 0.0 {
        return false;
    }

    #[cfg(feature = "dim2")]
    let sep3 = f32::MAX; // This case does not exist in 2D.
    #[cfg(feature = "dim3")]
    let sep3 = sat::cuboid_triangle_find_local_separating_edge_twoway(cube1, triangle2, &pos12).0;
    sep3 <= 0.0
}