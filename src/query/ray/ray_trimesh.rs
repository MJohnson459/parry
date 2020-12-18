use crate::bounding_volume::SimdAABB;
use crate::math::{Real, SimdBool, SimdReal, SIMD_WIDTH};
use crate::partitioning::{SimdBestFirstVisitStatus, SimdBestFirstVisitor};
use crate::query::{Ray, RayCast, RayIntersection, SimdRay};
use crate::shape::{FeatureId, TriMesh};
use simba::simd::{SimdBool as _, SimdPartialOrd, SimdValue};

impl RayCast for TriMesh {
    #[inline]
    fn cast_local_ray(&self, ray: &Ray, max_toi: Real, _: bool) -> Option<Real> {
        let mut visitor = TriMeshRayToiVisitor {
            mesh: self,
            ray,
            simd_ray: SimdRay::splat(*ray),
            max_toi,
        };

        self.quadtree()
            .traverse_best_first(&mut visitor)
            .map(|res| res.1)
    }

    #[inline]
    fn cast_local_ray_and_get_normal(
        &self,
        ray: &Ray,
        max_toi: Real,
        _: bool,
    ) -> Option<RayIntersection> {
        let mut visitor = TriMeshRayToiAndNormalVisitor {
            mesh: self,
            ray,
            simd_ray: SimdRay::splat(*ray),
            max_toi,
        };

        self.quadtree()
            .traverse_best_first(&mut visitor)
            .map(|(_, (best, mut res))| {
                // TODO: distinguish between the front and back faces?
                // if let FeatureId::Face(1) = res.feature {
                //     res.feature = FeatureId::Face(best + self.faces().len());
                // } else {
                res.feature = FeatureId::Face(best);
                // }

                res
            })
    }
}

/*
 * Costs functions.
 */
struct TriMeshRayToiVisitor<'a> {
    mesh: &'a TriMesh,
    ray: &'a Ray,
    simd_ray: SimdRay,
    max_toi: Real,
}

impl<'a> SimdBestFirstVisitor<u32, SimdAABB> for TriMeshRayToiVisitor<'a> {
    type Result = Real;

    #[inline]
    fn visit(
        &mut self,
        best: Real,
        aabb: &SimdAABB,
        data: Option<[Option<&u32>; SIMD_WIDTH]>,
    ) -> SimdBestFirstVisitStatus<Self::Result> {
        let (hit, toi) = aabb.cast_local_ray(&self.simd_ray, SimdReal::splat(self.max_toi));

        if let Some(data) = data {
            let mut weights = [0.0; SIMD_WIDTH];
            let mut mask = [false; SIMD_WIDTH];
            let mut results = [None; SIMD_WIDTH];

            let better_toi = toi.simd_lt(SimdReal::splat(best));
            let bitmask = (hit & better_toi).bitmask();

            for ii in 0..SIMD_WIDTH {
                if (bitmask & (1 << ii)) != 0 && data[ii].is_some() {
                    let triangle = self.mesh.triangle(*data[ii].unwrap());

                    if let Some(toi) = triangle.cast_local_ray(&self.ray, self.max_toi, true) {
                        results[ii] = Some(toi);
                        mask[ii] = true;
                        weights[ii] = toi;
                    }
                }
            }

            SimdBestFirstVisitStatus::MaybeContinue {
                weights: SimdReal::from(weights),
                mask: SimdBool::from(mask),
                results,
            }
        } else {
            SimdBestFirstVisitStatus::MaybeContinue {
                weights: toi,
                mask: hit,
                results: [None; SIMD_WIDTH],
            }
        }
    }
}

struct TriMeshRayToiAndNormalVisitor<'a> {
    mesh: &'a TriMesh,
    ray: &'a Ray,
    simd_ray: SimdRay,
    max_toi: Real,
}

impl<'a> SimdBestFirstVisitor<u32, SimdAABB> for TriMeshRayToiAndNormalVisitor<'a> {
    type Result = (u32, RayIntersection);

    #[inline]
    fn visit(
        &mut self,
        best: Real,
        aabb: &SimdAABB,
        data: Option<[Option<&u32>; SIMD_WIDTH]>,
    ) -> SimdBestFirstVisitStatus<Self::Result> {
        let (hit, toi) = aabb.cast_local_ray(&self.simd_ray, SimdReal::splat(self.max_toi));

        if let Some(data) = data {
            let mut weights = [0.0; SIMD_WIDTH];
            let mut mask = [false; SIMD_WIDTH];
            let mut results = [None; SIMD_WIDTH];

            let better_toi = toi.simd_lt(SimdReal::splat(best));
            let bitmask = (hit & better_toi).bitmask();

            for ii in 0..SIMD_WIDTH {
                if (bitmask & (1 << ii)) != 0 && data[ii].is_some() {
                    let triangle = self.mesh.triangle(*data[ii].unwrap());
                    if let Some(result) =
                        triangle.cast_local_ray_and_get_normal(&self.ray, self.max_toi, true)
                    {
                        results[ii] = Some((*data[ii].unwrap(), result));
                        mask[ii] = true;
                        weights[ii] = result.toi;
                    }
                }
            }

            SimdBestFirstVisitStatus::MaybeContinue {
                weights: SimdReal::from(weights),
                mask: SimdBool::from(mask),
                results,
            }
        } else {
            SimdBestFirstVisitStatus::MaybeContinue {
                weights: toi,
                mask: hit,
                results: [None; SIMD_WIDTH],
            }
        }
    }
}