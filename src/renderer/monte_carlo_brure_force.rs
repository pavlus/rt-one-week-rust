use super::{Hittable, RayCtx, Renderer};
use std::borrow::Borrow;
use crate::pdf::PDF;
use crate::SceneDesc;
use crate::random2::DefaultRng;
use crate::renderer::monte_carlo_importance::Shading;
use crate::scatter::Scatter::{Diffuse, Specular};
use crate::types::Color;

pub struct BruteForce;

impl<H: Hittable, I> Renderer<SceneDesc<H, I>> for BruteForce {
    fn color(&self, scene: &SceneDesc<H, I>, ray_ctx: &RayCtx, rng: &mut DefaultRng) -> Color {
        let mut shading: Vec<Shading> = Vec::new();
        let mut ray_ctx = ray_ctx.clone();
        let mut ttl = 0;
        loop {
            let hit = scene.hittable.hit(&ray_ctx, 0.000_001, 99999.0);
            if hit.is_none() {
                let result = scene.miss_shader.borrow()(&ray_ctx.ray);
                shading.push(Shading::emitted(result));
                break;
            }

            let hit = hit.unwrap();
            let emitted = if hit.normal.dot(&ray_ctx.ray.direction.normalize()) < 0.0 {
                hit.material.emmit(&hit)
            } else {
                Color::from_element(0.0)
            };

            let scatter = hit.material.scatter_with_pdf(ray_ctx.clone(), &hit);
            if scatter.is_none() {
                shading.push(Shading::emitted(emitted));
                break;
            }
            let scatter = scatter.unwrap();
            match scatter {
                Specular(scattered_ray, reflected) => {
                    ray_ctx = scattered_ray;
                    shading.push(Shading { emitted, reflected });
                }
                Diffuse(mat_pdf, albedo) => {
                    ray_ctx = ray_ctx.produce(hit.point, mat_pdf.generate(rng));
                    shading.push(Shading { emitted, reflected: albedo });
                }
            };
            ttl += 1;
            if ttl > scene.view.ttl { break; }
        }

        let mut result = shading.pop().unwrap().emitted;
        for shade in shading.into_iter().rev() {
            result = result.component_mul(&shade.reflected) + shade.emitted;
        }

        return result;
    }
}

/*
impl<H: Hittable, I> Renderer<SceneDesc<H, I>> for BruteForce {
    fn color(&self, scene: &SceneDesc<H, I>, ray_ctx: &RayCtx, rng: &mut DefaultRng) -> Color {
        let mut shading: Vec<Shading> = Vec::new();
        let mut ray_ctx = ray_ctx.clone();

        if ray_ctx.ttl <= 0 { return Color::from_element(0.0) };

        if let Some(hit) = scene.hittable.hit(&ray_ctx, 0.000_001, 99999.0) {
            let emitted = if hit.normal.dot(&ray_ctx.ray.direction.normalize()) < 0.0 {
                hit.material.emmit(&hit)
            } else {
                Color::from_element(0.0)
            };

            let _ray_ctx = ray_ctx.clone();
            if let Some(scatter) = hit
                .material
                .scatter_with_pdf(ray_ctx.clone(), &hit) {
                match scatter {
                    Scatter::Specular(ray_ctx, color) => {
                        emitted + color.component_mul(&self.color(&scene, &ray_ctx, rng))
                    }
                    Scatter::Diffuse(mat_pdf, albedo) => {
                        let scattered = _ray_ctx.produce(hit.point, mat_pdf.generate(rng));
                        // let weight = 1.0 / mat_pdf.value(&scattered.ray.direction, &hit);
                        let weight = 1.0;
                        emitted + weight * albedo.component_mul(&self.color(&scene, &scattered, rng))
                    }
                }
            } else {
                emitted
            }
        } else {
            scene.miss_shader.borrow()(&ray_ctx.ray)
        }
    }
}*/
