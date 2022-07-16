use super::*;

pub fn perlin_scene(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let perlin = with_rnd(|rnd| Perlin::new(rnd));
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
    Scene {
        view: get_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(HittableList::new(objs)),
            Box::new(NoHit),
            self::const_color_light,
            params,
        ),
    }
}
