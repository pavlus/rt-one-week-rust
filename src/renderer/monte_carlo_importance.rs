use std::borrow::Borrow;
use rand::distributions::Standard;
use rand::prelude::Distribution;

use crate::SceneDesc;
use crate::hittable::{Hit, Important};
use crate::pdf::{HittablePDF, MixturePDF, PDF};
use crate::random2::DefaultRng;
use crate::scatter::Scatter::{Diffuse, Specular};
use crate::types::{Color, Probability};

use super::{Hittable, RayCtx, Renderer};

#[derive(Clone)]
pub struct MonteCarlo {
    pub important_weight: Probability,
}

#[derive(Debug)]
enum RayType {
    Primary(RayCtx),
    Specular(RayCtx),
    Diffuse(RayCtx, Box<dyn PDF>),
    Light(RayCtx),
    Miss(RayCtx),
}

pub struct Shading {
    pub emitted: Color,
    pub reflected: Color,
}

impl Shading {
    pub fn emitted(emitted: Color) -> Shading {
        Shading {
            emitted,
            reflected: Color::zeros(),
        }
    }
}

impl<H: Hittable, I: Hittable + Important> Renderer<SceneDesc<H, I>> for MonteCarlo {

    fn color(&self, scene: &SceneDesc<H, I>, ray_ctx: &RayCtx, rng: &mut DefaultRng) -> Color {
        let mut hits: Vec<Hit> = Vec::new();
        let mut path: Vec<RayType> = Vec::new();
        let mut shading: Vec<Shading> = Vec::new();
        let mut ray_ctx = ray_ctx.clone();
        let mut ttl = 0;
        let primary = ray_ctx.clone();
        let mut has_diffuse = false;
        loop {
            let hit = scene.hittable.hit(&ray_ctx, 0.000_001, 99999.0);
            if hit.is_none() {
                let result = scene.miss_shader.borrow()(&ray_ctx.ray);
                shading.push(Shading::emitted(result));
                path.push(RayType::Miss(ray_ctx));
                break;
            }

            let hit = hit.unwrap();
            hits.push(hit.clone());
            let emitted = Self::emmit(&ray_ctx, &hit);
            let scatter = hit.material.scatter_with_pdf(ray_ctx.clone(), &hit);
            if scatter.is_none() {
                shading.push(Shading::emitted(emitted));
                path.push(RayType::Light(ray_ctx));
                break;
            }
            let scatter = scatter.unwrap();
            match scatter {
                Specular(_ray_ctx, reflected) => {
                    ray_ctx = _ray_ctx;
                    shading.push(Shading { emitted, reflected });
                    path.push(RayType::Specular(ray_ctx.clone()));
                }
                Diffuse(mat_pdf, attenuation) => {
                    let (weight, _ray_ctx) = self.try_connect(
                        ray_ctx, &hit, &scene.important, &mat_pdf, rng);
                    ray_ctx = _ray_ctx;
                    shading.push(Shading { emitted, reflected: attenuation * weight });
                    path.push(RayType::Diffuse(ray_ctx.clone(), mat_pdf));
                    has_diffuse = true;
                }
            };
            ttl += 1;
            if ttl > scene.view.ttl { break; }
        }

        // todo: try mutate paths

        if false && has_diffuse && Distribution::<Probability>::sample(&Standard, rng) < 0.0000001 {
            //debug
            eprintln!("Path: {:?}", &path);
        }

        let mut result = shading.pop().unwrap().emitted;
        for shade in shading.into_iter().rev() {
            result = result.component_mul(&shade.reflected) + shade.emitted;
        }

        return result;
    }
}

impl MonteCarlo {
    fn emmit(ray_ctx: &RayCtx, hit: &Hit) -> Color {
        if hit.normal.dot(&ray_ctx.ray.direction.normalize()) < 0.0 {
            hit.material.emmit(&hit)
        } else {
            Color::from_element(0.0)
        }
    }

    fn try_connect<I: Hittable + Important, P: PDF + ?Sized>(
        &self,
        ray_ctx: RayCtx,
        hit: &Hit,
        important: &I,
        mat_pdf: &P,
        rng: &mut DefaultRng
    ) -> (Probability, RayCtx) {
        let hittable_pdf = HittablePDF::new(&hit.point, important);
        let mixture = MixturePDF::new(
            &hittable_pdf,
            mat_pdf,
            self.important_weight,
        );
        let mut ray_ctx = ray_ctx.produce(hit.point, mixture.generate(rng));
        let scatter_pdf = mat_pdf.value(&ray_ctx.ray.direction, &hit);
        let mixture_pdf = mixture.value(&ray_ctx.ray.direction, &hit);
        let mut weight = scatter_pdf / mixture_pdf;
        if weight.is_nan() || weight.is_infinite() {
            ray_ctx.ray.direction = mat_pdf.generate(rng);
            weight = 1.0;
        }
        (weight, ray_ctx)
    }

}

