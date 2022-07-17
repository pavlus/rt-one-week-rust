use super::*;

pub fn perlin_scene(timespan: Timespan, params: &Params) -> SceneDesc<Vec<Box<dyn Hittable>>, NoHit> {
    let mut rng = DefaultRng::default();
    let perlin = Perlin::new(&mut rng);
    let mut objs: Vec<Box<dyn Hittable>> = vec![];
    objs.push(Box::new(
        Sphere::new(P3::new(0.0, -1000.0, 0.0), 1000.0,
                    Lambertian::texture(PerlinTexture::new(
                        Box::new(move |p, scale| perlin.noise(&(scale * &p.coords).into()) * 0.5 + 0.5), 4.0,
                    )))));
    objs.push(Box::new(
        Sphere::new(P3::new(0.0, 2.0, 0.0), 2.0,
                    Lambertian::texture(PerlinTexture::new(
                        Box::new(move |p, scale| perlin.turb(&(scale * &p.coords))), 4.0,
                    )))));
    objs.push(Box::new(
        Sphere::new(P3::new(0.0, 2.0, 4.0), 2.0,
                    Lambertian::texture(PerlinTexture::new(
                        Box::new(move |p, scale| 0.5 * (1.0 + perlin.turb(&(scale * &p.coords)))), 4.0,
                    )))));
    objs.push(Box::new(
        Sphere::new(P3::new(0.0, 2.0, -4.0), 2.0,
                    Lambertian::texture(PerlinTexture::new(
                        Box::new(move |p, scale| 0.5 * (1.0 + ((scale * p.z) as ColorComponent + 10.0 * perlin.turb(&p.coords)).sin())),
                        5.0,
                    )))));
    SceneDesc {
        view: get_cam(params.width, params.height, timespan, params.bounces as i32),
        hittable: objs,
        important: NoHit,
        miss_shader: const_color_light,
    }
}
