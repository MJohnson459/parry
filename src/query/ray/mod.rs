//! Ray-casting related definitions and implementations.

#[doc(inline)]
pub use self::ray::{Ray, RayCast, RayIntersection};
pub use self::ray_ball::ray_toi_with_ball;
pub use self::ray_halfspace::{line_toi_with_halfspace, ray_toi_with_halfspace};
pub use self::ray_support_map::local_ray_intersection_with_support_map_with_params;
#[cfg(feature = "dim3")]
pub use self::ray_triangle::local_ray_intersection_with_triangle;
pub use self::simd_ray::SimdRay;

#[doc(hidden)]
pub mod ray;
mod ray_aabb;
mod ray_ball;
mod ray_bounding_sphere;
mod ray_cuboid;
mod ray_halfspace;
mod ray_heightfield;
mod ray_polyline;
mod ray_support_map;
mod ray_triangle;
mod ray_trimesh;
mod simd_ray;