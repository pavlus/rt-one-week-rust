use rand::distributions::{Distribution, Standard};
use rand::distributions::uniform::SampleRange;
use rand::{Rng, RngCore};
use crate::bvh::{BoundedHittable, BVHIndexed};
use crate::hittable::{AABoxMono};
use crate::material::PolishedMetal;
use super::*;

pub fn weekend_final(complexity: i8, timespan: Timespan, params: &Params) -> SceneDesc<Box<BVHIndexed<Box<dyn BoundedHittable>>>, NoHit> {
    let mut rng = DefaultRng::default();
    let mut objs: Vec<Box<dyn BoundedHittable>> = vec![
        Box::new(Sphere::new(P3::new(0.0, -1000.0, 0.0), 1000.0,
                             Lambertian::texture(Checker::new(
                                 Color::new(0.2, 0.3, 0.1),
                                 Color::new(0.9, 0.9, 0.9), 10.0,
                             )))),
        Box::new(Sphere::new(P3::new(4.0, 1.0, 0.0), 1.0, PolishedMetal::new(Color::new(0.7, 0.6, 0.5)))),
        Box::new(Sphere::new(P3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5))),
        Box::new(Sphere::new(P3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::<Color>::new(Color::new(0.8, 0.8, 0.9)))),
    ];

    for a in -complexity..=complexity {
        for b in -complexity..=complexity {
            let center = P3::new(0.9 * rnd_geometry(&mut rng) + a as Geometry,
                                 0.2,
                                 0.9 * rnd_geometry(&mut rng) + b as Geometry);

            if (&center - P3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                objs.push(
                    #[allow(overlapping_range_endpoints)]
                    match rng.next_u32() % 100 {
                        0..=80 => Box::new(MovingSphere::new(
                            center,
                            &center + V3::new(0.0, rnd_geometry(&mut rng) * 0.5, 0.0),
                            0.0..1.0, 0.2,
                            Lambertian::<Color>::new(rnd_color(&mut rng).component_mul(&rnd_color(&mut rng))),
                        )),
                        80..=95 => Box::new(Sphere::new(
                            center,
                            0.2,
                            PolishedMetal::new(0.5 * (rnd_color(&mut rng).add_scalar(1.0))),
                        )),
                        _ => Box::new(Sphere::new(center, 0.2,
                                                  Dielectric::new(1.5)))
                    });
            }
        }
    }
    SceneDesc {
        view: get_cam(params.width, params.height, timespan.clone(), params.bounces as i32),
        hittable: Box::new(BVHIndexed::new(objs, timespan)),
        important: NoHit,
        miss_shader: sky,
    }
}

pub fn next_week(timespan: Timespan, params: &Params) -> SceneDesc<BVHIndexed<Box<dyn BoundedHittable>>, XZRect<DiffuseLight<Color>>> {
    let mut rng = DefaultRng::default();
    let nb = 20;
    let ground = Lambertian::<Color>::new(Color::new(0.48, 0.83, 0.53));

    let mut boxes: Vec<AABoxMono<_>> = vec![];
    for i in 0..nb {
        for j in 0..nb {
            let w = 100.0;
            let x0 = -1000.0 + i as Geometry * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as Geometry * w;
            let x1 = x0 + w;
            let y1 = (0.0..100.0).sample_single(&mut rng);
            let z1 = z0 + w;
            boxes.push(AABox::mono(x0..x1, y0..y1, z0..z1, ground.clone()));
        }
    }
    let boxes = Box::new(BVHIndexed::new(boxes, timespan.clone()));

    let light_mat = DiffuseLight::new(Color::from_element(1.0), 7.0);
    let light = XZRect::new(123.0..423.0, 147.0..412.0, 554.0, light_mat);

    let center = P3::new(450.0, 400.0, 200.0);
    let moving_sphere = Box::new(MovingSphere::new(
        center, &center + V3::new(80.0, 0.0, 0.0),
        0.0..1.0,
        50.0,
        Lambertian::<Color>::new(Color::new(0.7, 0.3, 0.1))));


    let fuzzy_ball = Box::new(Sphere::new(
        P3::new(0.0, 150.0, 145.0), 50.0,
        GlossyMetal::new(Color::new(0.8, 0.8, 0.9), 10.0)));

    let glass_ball = Box::new(Sphere::new(
        P3::new(250.0, 450.0, 300.0), 60.0,
        Dielectric::new(1.5)));


    let mat = Dielectric::new(1.5);
    let center = P3::new(180.0, 150.0, 145.0);
    let radius = 60.0;
    let blue_ball_surface = Box::new(
        Sphere::new(center, radius, mat));
    let blue_ball_subsurface = Box::new(ConstantMedium::new(
        Sphere::new(center, radius, NoMat),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));
    let scene_fog = Box::new(ConstantMedium::new(
        Sphere::radius(5000.0, NoMat),
        0.00011,
        Color::from_element(1.0),
    ));
    let stone_ball = Box::new(Sphere::new(
        P3::new(400.0, 200.0, 400.0),
        100.0,
        Lambertian::texture(ImageTexture::load("./textures/stone.png")))
    );

    let perlin = Perlin::new(&mut rng);
    let perlin_ball = Box::new(Sphere::new(P3::new(220.0, 280.0, 300.0), 80.0,
                                           Lambertian::texture(PerlinTexture::new(
                                               Box::new(move |p, scale| 0.5 * (1.0 + ((scale * p.x) as ColorComponent + 2.0 * perlin.turb(&(p * scale).coords)).sin())),
                                               0.1,
                                           ))));
    let mut foam_box: Vec<Sphere<_>> = vec![];
    for _ in 0..1000 {
        foam_box.push(Sphere::new(
            (165.0 * rnd_v3(&mut rng)).into(),
            10.0,
            Lambertian::<Color>::new(Color::from_element(0.73)),
        ));
    }
    let foam_box = Box::new(BVHIndexed::new(foam_box, timespan.clone())
        .rotate_y(15.0)
        .translate(V3::new(-200.0, 300.0, 335.0))
    );

    let mut objs: Vec<Box<dyn BoundedHittable>> = vec![];
    objs.push(Box::new(light.clone().flip_normals().clone()));
    objs.push(moving_sphere);
    objs.push(fuzzy_ball);
    objs.push(glass_ball.clone());
    objs.push(stone_ball);
    objs.push(perlin_ball);
    objs.push(blue_ball_subsurface);
    objs.push(blue_ball_surface);
    objs.push(scene_fog);
    objs.push(boxes);
    objs.push(foam_box);
    SceneDesc {
        view: next_week_cam(params.width, params.height, timespan.clone(), params.bounces as i32),
        hittable: BVHIndexed::new(objs, timespan),
        important: light,
        miss_shader: |_| Color::new(0.0, 0.0, 0.001),
    }
}

fn rnd_v3<R: Rng + ?Sized>(rng: &mut R) -> V3 {
    V3::new(Standard.sample(rng), Standard.sample(rng), Standard.sample(rng))
}
fn rnd_color<R: Rng + ?Sized>(rng: &mut R) -> Color {
    Color::new(Standard.sample(rng), Standard.sample(rng), Standard.sample(rng))
}

fn rnd_geometry<R: Rng + ?Sized>(rng: &mut R) -> Geometry {
    Standard.sample(rng)
}



fn next_week_cam(nx: u32, ny: u32, timespan: Timespan, ttl: i32) -> View {
    let aspect = (nx as Geometry) / (ny as Geometry);
    let from = V3::new(478.0, 278.0, -680.0);
    // let at = V3::new(278.0, 170.0, 40.0);
    let at = V3::new(278.0, 300.0, 0.0);

    let dist_to_focus = 2.0;
    let aperture = 0.00;
    // let vfov = 22.0;
    let vfov = 62.0;

    View::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        vfov,
        aspect,
        dist_to_focus,
        aperture,
        timespan,
        ttl,
    )
}
