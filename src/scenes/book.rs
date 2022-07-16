use crate::hittable::{IsometryT, Rotate};
use super::*;

pub fn weekend_final(complexity: i8, t_off: Time, t_span: Time, params: &Params) -> Scene {
    let mut objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(P3::new(0.0, -1000.0, 0.0), 1000.0,
                             Lambertian::texture(Checker::new(
                                 Color::new(0.2, 0.3, 0.1),
                                 Color::new(0.9, 0.9, 0.9), 10.0,
                             )))),
        Box::new(Sphere::new(P3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Color::new(0.7, 0.6, 0.5)))),
        Box::new(Sphere::new(P3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5))),
        Box::new(Sphere::new(P3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::<Color>::new(Color::new(0.8, 0.8, 0.9)))),
    ];

    for a in -complexity..=complexity {
        for b in -complexity..=complexity {
            let center = P3::new(0.9 * next_std_distance() + a as Geometry,
                                 0.2,
                                 0.9 * next_std_distance() + b as Geometry);

            if (&center - P3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                objs.push(
                    #[allow(overlapping_range_endpoints)]
                    match next_std_u32() % 100 {
                        0..=80 => Box::new(MovingSphere::new(
                            center,
                            &center + V3::new(0.0, next_std_distance() * 0.5, 0.0),
                            0.0..1.0, 0.2,
                            Box::new(Lambertian::<Color>::new(next_color().component_mul(&next_color()))),
                        )),
                        80..=95 => Box::new(Sphere::new(
                            center,
                            0.2,
                            Metal::new(0.5 * (next_color().add_scalar(1.0))),
                        )),
                        _ => Box::new(Sphere::new(center, 0.2,
                                                  Dielectric::new(1.5)))
                    });
            }
        }
    }
    Scene {
        view: get_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            BVH::new(objs),
            Box::new(NoHit),
            self::sky,
            params,
        ),
    }
}

pub fn next_week(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let nb = 20;
    let ground = Lambertian::<Color>::new(Color::new(0.48, 0.83, 0.53));

    let mut boxes: Vec<Box<dyn Hittable>> = vec![];
    for i in 0..nb {
        for j in 0..nb {
            let w = 100.0;
            let x0 = -1000.0 + i as Geometry * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as Geometry * w;
            let x1 = x0 + w;
            let y1 = 100.0 * (next_std_distance() + 0.001);
            let z1 = z0 + w;
            boxes.push(Box::new(AABox::mono(x0..x1, y0..y1, z0..z1, ground.clone())));
        }
    }
    let boxes = BVH::new(boxes);

    let light_mat = DiffuseLight::new(Color::from_element(1.0), 7.0);
    let light = Box::new(XZRect::new(123.0..423.0, 147.0..412.0, 554.0, light_mat).flip_normals());

    let center = P3::new(450.0, 400.0, 200.0);
    let moving_sphere = Box::new(MovingSphere::new(
        center, &center + V3::new(80.0, 0.0, 0.0),
        0.0..1.0,
        50.0,
        Lambertian::<Color>::new(Color::new(0.7, 0.3, 0.1))));


    let fuzzy_ball = Box::new(Sphere::new(
        P3::new(0.0, 150.0, 145.0), 50.0,
        Metal::new_fuzzed(Color::new(0.8, 0.8, 0.9), 10.0)));

    let glass_ball = Box::new(Sphere::new(
        P3::new(250.0, 450.0, 300.0), 60.0,
        Dielectric::new(1.5)));


    let mat = Dielectric::new(1.5);
    let center = P3::new(180.0, 150.0, 145.0);
    let radius = 60.0;
    let blue_ball_surface = Box::new(
        Sphere::new( center, radius, mat));
    let blue_ball_subsurface = Box::new(ConstantMedium::new(
        Sphere::new(center, radius, NoMat),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));
    let scene_fog = Box::new(ConstantMedium::new(
        Sphere::new(P3::default(), 5000.0, NoMat),
        0.00011,
        Color::from_element(1.0),
    ));
    let stone_ball = Box::new(Sphere::new(
        P3::new(400.0, 200.0, 400.0),
        100.0,
        Lambertian::texture(ImageTexture::load("./textures/stone.png")))
    );

    let perlin = with_rnd(|rnd| Perlin::new(rnd));
    let perlin_ball = Box::new(Sphere::new(P3::new(220.0, 280.0, 300.0), 80.0,
                                 Lambertian::texture(PerlinTexture::new(
                                     Box::new(move |p, scale| 0.5 * (1.0 + ((scale * p.x) as ColorComponent + 2.0 * perlin.turb(&(p * scale).coords)).sin())),
                                     0.1,
                                 ))));

    let mut foam_box: Vec<Box<dyn Hittable>> = vec![];
    for _ in 0..1000 {
        foam_box.push(Box::new(Sphere::new(
            (165.0 * V3::new(next_std(), next_std(), next_std())).into(),
            10.0,
            Lambertian::<Color>::new(Color::from_element(0.73)),
        )));
    }
    let foam_box = Box::new(BVH::new(foam_box)
        .rotate_y(15.0)
        .translate(V3::new(-200.0, 300.0, 335.0))
    );

    let mut objs: Vec<Box<dyn Hittable>> = vec![];
    objs.push(light.clone());
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
    let mut important: HittableList<Box<dyn Hittable>> = HittableList::empty();
    for _ in 0..20 { important.push(light.clone()); }
    important.push(glass_ball);
    Scene {
        view: next_week_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            BVH::new(objs),
            Box::new(important),
            |_| Color::new(0.0, 0.0, 0.001),
            params,
        ),
    }
}


fn next_week_cam(nx: u32, ny: u32, t_off: Time, t_span: Time, ttl: i32) -> View {
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
        t_off..t_span,
        ttl,
    )
}
