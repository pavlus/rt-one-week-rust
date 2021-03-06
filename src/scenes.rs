use std::sync::Arc;

use crate::hittable::{AABox, ConstantMedium, Hittable, HittableList, RotateYOp, FlipNormalsOp, TranslateOp, MovingSphere, Sphere, XYRect, XZRect, YZRect, NoHit, AABoxMono, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::noise::Perlin;
use crate::random::{next_color, next_std_f64, with_rnd, next_std_u32};
use crate::texture::{Checker, Color, ImageTexture, PerlinTexture};
use crate::vec::V3;
use crate::camera::Camera;
use crate::renderer::{Renderer, RendererImpl, RendererType};
use crate::ray::Ray;
use crate::bvh::BVH;

pub struct Scene {
    pub camera: Camera,
    pub renderer: RendererImpl,
}

impl Scene {
    pub fn color(&self, u: f64, v: f64) -> V3 {
        self.renderer.color(&self.camera.get_ray(u, v))
    }
}

pub fn perlin_scene(r_type: RendererType, nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Scene {
    let perlin = with_rnd(|rnd| Perlin::new(rnd));
    let mut objs: Vec<Box<dyn Hittable>> = vec![];
    objs.push(Box::new(
        Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0,
                    Lambertian::texture(Box::new(PerlinTexture::new(
                        Box::new(move |p, scale| perlin.noise(scale * p) * 0.5 + 0.5), 4.0,
                    ))))));
    objs.push(Box::new(
        Sphere::new(V3::new(0.0, 2.0, 0.0), 2.0,
                    Lambertian::texture(Box::new(PerlinTexture::new(
                        Box::new(move |p, scale| perlin.turb(scale * p)), 4.0,
                    ))))));
    objs.push(Box::new(
        Sphere::new(V3::new(0.0, 2.0, 4.0), 2.0,
                    Lambertian::texture(Box::new(PerlinTexture::new(
                        Box::new(move |p, scale| 0.5 * (1.0 + perlin.turb(scale * p))), 4.0,
                    ))))));
    objs.push(Box::new(
        Sphere::new(V3::new(0.0, 2.0, -4.0), 2.0,
                    Lambertian::texture(Box::new(PerlinTexture::new(
                        Box::new(move |p, scale| 0.5 * (1.0 + (scale * p.z + 10.0 * perlin.turb(p)).sin())),
                        5.0,
                    ))))));
    Scene {
        camera: get_cam(nx, ny, t_off, t_span, ttl),
        renderer: RendererImpl::pick_renderer(
            r_type,
            Box::new(HittableList::new(objs)),
            Box::new(NoHit),
            self::const_color_light,
            ttl,
        ),
    }
}

pub fn img_scene(r_type: RendererType, nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Scene {
    let mut objs: Vec<Box<dyn Hittable>> = Vec::new();
    objs.push(Box::new(
        Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0,
                    Lambertian::texture(Box::new(
                        Checker::new(
                            Color::new(0.0, 0.0, 0.0),
                            Color::new(1.0, 1.0, 1.0),
                            10.0
                        ))))));
    objs.push(Box::new(Sphere::new(V3::new(0.0, 2.0, 0.0), 2.0,
        Lambertian::texture(Box::new(ImageTexture::load("./textures/stone.png"))))));

    Scene {
        camera: get_cam(nx, ny, t_off, t_span, ttl),
        renderer: RendererImpl::pick_renderer(
            r_type,
            Box::new(HittableList::new(objs)),
            Box::new(NoHit),
            self::const_color_light,
            ttl
        ),
    }
}

pub fn img_lit_scene(r_type: RendererType, nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Scene {
    let light = Sphere::new(V3::new(0.0, 3.0, -2.0), 2.0,
        DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 0.99)), 2.0));
    let light1 = Sphere::new(V3::new(0.0, 3.0, -2.0), 2.0,
        DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 0.99)), 2.0));
    let objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0,
                             Lambertian::texture(Box::new(Checker::new(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0), 10.0))))),
        Box::new(Sphere::new(V3::new(0.0, 2.0, 2.0), 2.0,
                             Lambertian::texture(Box::new(ImageTexture::load("./textures/stone.png"))))),
        Box::new(light),
    ];
    Scene {
        camera: get_cam(nx, ny, t_off, t_span, ttl),
        renderer: RendererImpl::pick_renderer(
            r_type,
            Box::new(HittableList::new(objs)),
            Box::new(light1),
            self::const_color_dark,
            ttl
        ),
    }
}

#[allow(dead_code)]
pub fn img_lit_rect_scene(r_type: RendererType, nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Scene {
    let l1 = Box::new(XZRect::new(-1.0..1.0, -1.0..1.0, 2.5, Arc::new(
        DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 0.99)), 4.0))));
    let l2 = Box::new(XYRect::new(-1.0..1.0, 0.5..1.5, -1.5, Arc::new(
        DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 0.99)), 4.0))));
    let objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0,
                             Lambertian::texture(Box::new(Checker::new(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0), 10.0))))),
        Box::new(Sphere::new(V3::new(0.0, 1.0, 0.0), 1.0,
                             Lambertian::texture(Box::new(ImageTexture::load("./textures/stone.png"))))),
        l1.clone(),
        l2.clone(),
    ];
    Scene {
        camera: get_cam(nx, ny, t_off, t_span, ttl),
        renderer: RendererImpl::pick_renderer(
            r_type,
            Box::new(HittableList::new(objs)),
            Box::new(HittableList::new(vec![l1, l2])),
            self::const_color_dark,
            ttl
        ),
    }
}

fn cornel_box_prototype() -> Vec<Box<dyn Hittable>> {
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let floor_white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ceil_white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let back_white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 1.0)), 15.0));

    vec![
        Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 555.0, green).flip_normals()),
        Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 0.0, red)),
        Box::new(XZRect::new(213.0..343.0, 227.0..332.0, 554.0, light)),
        Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 0.0, floor_white)),
        Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 555.0, ceil_white).flip_normals()),
        Box::new(XYRect::new(0.0..555.0, 0.0..555.0, 555.0, back_white).flip_normals()),
    ]
}

pub fn cornel_box_with_instances(r_type: RendererType, nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Scene {
    let mut objs = cornel_box_prototype();
    objs.push(
        Box::new(AABox::mono(0.0..165.0, 0.0..165.0, 0.0..165.0,
                             Arc::new(Lambertian::new(Color(V3::all(0.73)))))
            .rotate_y(-18.0)
            .translate(V3::new(130.0, 0.0, 65.0))
        ));

    let shiny_box = Box::new(AABox::mono(0.0..165.0, 0.0..330.0, 0.0..165.0,
                                 // Arc::new(Lambertian::new(Color(V3::all(0.73)))))
                                 Arc::new(Metal::new(V3::all(1.0))))
        .rotate_y(15.0)
        .translate(V3::new(265.0, 0.0, 295.0))
    );
    objs.push(shiny_box.clone());

    let light_mat = Arc::new(DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 1.0)), 15.0));
    let light = XZRect::new(213.0..343.0, 227.0..332.0, 554.0, light_mat);
    objs.push(Box::new(light.clone().flip_normals()));
    objs.swap_remove(2);

    Scene {
        camera: cornel_box_cam(nx, ny, t_off, t_span, ttl),
        renderer: RendererImpl::pick_renderer(
            r_type,
            Box::new(HittableList::new(objs)),
            Box::new(HittableList::new(vec![shiny_box, Box::new(light)])),
            self::const_color_black,
            ttl
        ),
    }
}

pub fn cornel_box_with_is(r_type: RendererType, nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Scene {
    let mut objs = cornel_box_prototype();
    let mut important: Vec<Box<dyn Hittable>> = vec![];
    /*objs.push(
        AABox::mono(0.0..165.0, 0.0..165.0, 0.0..165.0,
                    Arc::new(Lambertian::new(Color(V3::all(0.73)))))
            .rotate_y(-18.0)
            .translate(V3::new(130.0, 0.0, 65.0))
    );*/
    let fog = Box::new(ConstantMedium::new(
        // Box::new(AABox::mono(0.0..165.0, 0.0..165.0, 0.0..165.0,
        Box::new(AABox::mono(0.0..165.0, 0.0..165.0, 0.0..165.0,
                             // todo: null-texture/null-material?
                             Arc::new(Lambertian::new(Color(V3::new(1.0, 0.0, 1.0)))))
            .translate(V3::new(130.0, 0.0, 65.0))
            .rotate_y(-18.0)
        ),
        0.01,
        Color::new(1.0, 1.0, 1.0))
    );

    let lamb = Arc::new(Lambertian::new(Color(V3::all(0.73))));
    let metal = Arc::new(Metal::new(V3::all(1.0)));
    let shiny_box =
        AABox::new(0.0..165.0, 0.0..330.0, 0.0..165.0,
                       // AABox::new(265.0..(165.0+265.0), 0.0..330.0, 295.0..(165.0+265.0),
                       lamb.clone(),
                       lamb.clone(),
                       lamb.clone(),
                       lamb.clone(),
                       // lamb.clone(),
                       metal.clone(),
                       lamb.clone(),
    )
        .rotate_y(15.0)
        // .rotate_y(-90.0)
        // .translate(V3::new(265.0, 80.0, 295.0))
        .translate(V3::new(265.0, 0.0, 295.0))
        ;

    let light_mat = Arc::new(DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 1.0)), 15.0));
    let light = Box::new(XZRect::new(213.0..343.0, 227.0..332.0, 554.0, light_mat).flip_normals());
    objs.push(light.clone());
    objs.swap_remove(2);

    let sphere: Box<Translate<Sphere<Dielectric>>> = Box::new(
        Sphere::new(
            V3::new(-87.5, 87.5, -12.5),
            88.5,
            Dielectric::new(1.5),
        )
            .translate(V3::new(130.0, 0.0, 65.0))
            .translate(V3::new(165.0, 165.0, 165.0))
    );

    objs.push(fog.clone());
    objs.push(Box::new(shiny_box.clone()));
    objs.push(sphere.clone());

    important.push(fog);
    important.push(Box::new(shiny_box.clone()));
    important.push(sphere);
    important.push(light);
    let important = Box::new(HittableList::new(important));

    Scene {
        camera: cornel_box_cam(nx, ny, t_off, t_span, ttl),
        renderer: RendererImpl::pick_renderer(
            r_type,
            Box::new(HittableList::new(objs)),
            important,
            self::const_color_black,
            ttl
        ),
    }
}

pub fn cornel_box_volumes(r_type: RendererType, nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Scene {
    let mut objs = cornel_box_prototype();
    objs.push(Box::new(ConstantMedium::new(
        AABox::mono(0.0..165.0, 0.0..165.0, 0.0..165.0,
                             // todo: null-texture/null-material?
                             Arc::new(Lambertian::new(Color(V3::new(1.0, 0.0, 1.0)))))
            .rotate_y(-18.0)
            .translate(V3::new(130.0, 0.0, 65.0)),
        0.01,
        Color::new(1.0, 1.0, 1.0)
    )));

    objs.push(Box::new(ConstantMedium::new(
        AABox::mono(0.0..165.0, 0.0..330.0, 0.0..165.0,
                             Arc::new(Lambertian::new(Color(V3::new(0.0, 1.0, 0.0)))))
            .rotate_y(15.0)
            .translate(V3::new(265.0, 0.0, 295.0)),
        0.01,
        Color::new(0.0, 0.0, 0.0)
    )));

    let light_mat = Arc::new(DiffuseLight::new(Box::new(Color::new(1.0, 1.0, 1.0)), 7.0));
    let light = XZRect::new(213.0..343.0, 227.0..332.0, 554.0, light_mat);
    objs.push(Box::new(light.clone().flip_normals()));
    objs.swap_remove(2);


    Scene {
        camera: cornel_box_cam(nx, ny, t_off, t_span, ttl),
        renderer: RendererImpl::pick_renderer(
            r_type,
            Box::new(HittableList::new(objs)),
            Box::new(light),
            self::const_color_dark,
            ttl,
        )
    }
}

pub fn weekend_final(r_type: RendererType, complexity: i8, nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Scene {
    let mut objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(V3::new(0.0, -1000.0, 0.0), 1000.0,
            Lambertian::texture(Box::new(Checker::new(
                Color::new(0.2, 0.3, 0.1),
                Color::new(0.9, 0.9, 0.9), 10.0,
            ))))),
        Box::new(Sphere::new(V3::new(4.0, 1.0, 0.0), 1.0, Metal::new(V3::new(0.7, 0.6, 0.5)))),
        Box::new(Sphere::new(V3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5))),
        Box::new(Sphere::new(V3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(Color::new(0.8, 0.8, 0.9)))),
    ];

    for a in -complexity..=complexity {
        for b in -complexity..=complexity {
            let center = V3::new(0.9 * next_std_f64() + a as f64,
                                 0.2,
                                 0.9 * next_std_f64() + b as f64);

            if (center - V3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                objs.push(
                    match next_std_u32() % 100 {
                        0..=80 => Box::new(MovingSphere::new(
                            center,
                            center + V3::new(0.0, 0.5 * next_std_f64(), 0.0),
                            0.0, 1.0, 0.2,
                            Box::new(Lambertian::new(Color(next_color() * next_color()))),
                        )),
                        80..=95 => Box::new(Sphere::new(
                            center,
                            0.2,
                            Metal::new(0.5 * (next_color() + 1.0)),
                        )),
                        _ => Box::new(Sphere::new(center, 0.2,
                                                  Dielectric::new(1.5)))
                    });
            }
        }
    }
    Scene {
        camera: get_cam(nx, ny, t_off, t_span, ttl),
        renderer: RendererImpl::pick_renderer(
            r_type,
            BVH::new(objs),
            Box::new(NoHit),
            self::sky,
            ttl
        ),
    }
}

pub fn next_week(r_type: RendererType, nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Scene {
    let nb = 20;
    let ground = Arc::new(Lambertian::new(Color(V3::new(0.48, 0.83, 0.53))));
    let mut objs: Vec<Box<dyn Hittable>> = vec![];
    let mut boxes: Vec<Box<dyn Hittable>> = vec![];
    for i in 0..nb {
        for j in 0..nb {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as f64 * w;
            let x1 = x0 + w;
            let y1 = 100.0 * (next_std_f64() + 0.001);
            let z1 = z0 + w;
            boxes.push(Box::new(AABox::mono(x0..x1, y0..y1, z0..z1, ground.clone())));
        }
    }
    objs.push(BVH::new(boxes));

    let light_mat = Arc::new(DiffuseLight::new(Box::new(Color(V3::ones())), 7.0));
    let light = XZRect::new(123.0..423.0, 147.0..412.0, 554.0, light_mat);
    objs.push(Box::new(light.clone().flip_normals()));

    let center = V3::new(400.0, 400.0, 200.0);
    objs.push(Box::new(MovingSphere::new(
        center, center + V3::new(30.0, 0.0, 0.0),
        0.0, 1.0,
        50.0,
        Box::new(Lambertian::new(
            Color(V3::new(0.7, 0.3, 0.1))
        )))));

    objs.push(Box::new(Sphere::new(
        V3::new(0.0, 150.0, 145.0), 50.0,
        Metal::new_fuzzed(V3::new(0.8, 0.8, 0.9), 10.0))));

    let mat = Dielectric::new(1.5);
    objs.push(Box::new(Sphere::new(
        V3::new(260.0, 150.0, 45.0), 50.0,
        mat)));
    objs.push(Box::new(ConstantMedium::new(
        Sphere::new(V3::new(260.0, 150.0, 45.0), 50.0, mat),
        0.2,
        Color(V3::new(0.2, 0.4, 0.9)),
    )));
    objs.push(Box::new(ConstantMedium::new(
        Box::new(Sphere::new(V3::zeros(), 5000.0, mat)),
        0.0001,
        Color(V3::ones()),
    )));
    objs.push(Box::new(Sphere::new(
        V3::new(400.0, 200.0, 400.0),
        100.0,
        Lambertian::texture(Box::new(ImageTexture::load("./textures/stone.png"))))
    ));

    let perlin = with_rnd(|rnd| Perlin::new(rnd));
    objs.push(Box::new(Sphere::new(V3::new(220.0, 280.0, 300.0), 80.0,
        Lambertian::texture(Box::new(PerlinTexture::new(
            Box::new(move |p, scale| 0.5 * (1.0 + (scale * p.x + 2.0 * perlin.turb(scale * p)).sin())),
            0.1,
        ))))));

    let mut foam_box: Vec<Box<dyn Hittable>> = vec![];
    for _ in 0..1000 {
        foam_box.push(Box::new(Sphere::new(
            165.0 * V3::new(next_std_f64(), next_std_f64(), next_std_f64()),
            10.0,
            Lambertian::new(Color(V3::all(0.73))),
        )));
    }
/*    objs.push(
        BVH::new(foam_box)
            .rotate_y(15.0)
            .translate(V3::new(-100.0, 270.0, 395.0))
    );*/
    Scene {
        camera: next_week_cam(nx, ny, t_off, t_span, ttl),
        renderer: RendererImpl::pick_renderer(
            r_type,
            BVH::new(objs),
            Box::new(light.flip_normals()),
            |_| V3::zeros(),
            ttl
        ),
    }
}

fn cornel_box_cam(nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Camera {
    let aspect = (nx as f64) / (ny as f64);
    let from = V3::new(278.0, 278.0, -680.0);
    let at = V3::new(278.0, 278.0, 0.0);

    let dist_to_focus = 2.0;
    let aperture = 0.00;
    let vfov = 80.0;
    Camera::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        vfov,
        aspect,
        dist_to_focus,
        aperture,
        t_off, t_span,
        ttl,
    )
}

fn get_cam(nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Camera {
    let aspect = (nx as f64) / (ny as f64);
    let from = V3::new(13.0, 2.0, 3.0);
    let at = V3::new(0.0, 0.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    Camera::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        vfov,
        aspect,
        dist_to_focus,
        aperture,
        t_off, t_span,
        ttl,
    )
}

fn closeup_cam(nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Camera {
    let aspect = (nx as f64) / (ny as f64);
    let from = V3::new(-3.0, 3.0, 2.0);
    let at = V3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (from - at).length();
    let aperture = 0.01;
    Camera::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        /*  vfov*/ 80.0,
        aspect,
        dist_to_focus,
        aperture,
        t_off, t_span,
        ttl,
    )
}


fn next_week_cam(nx: u32, ny: u32, t_off: f32, t_span: f32, ttl: i32) -> Camera {
    let aspect = (nx as f64) / (ny as f64);
    let from = V3::new(478.0, 278.0, -680.0);
    // let at = V3::new(278.0, 170.0, 40.0);
    let at = V3::new(278.0, 300.0, 0.0);

    let dist_to_focus = 2.0;
    let aperture = 0.00;
    // let vfov = 22.0;
    let vfov = 62.0;

    Camera::new_look(
        from, at,
        /*    up*/ V3::new(0.0, 1.0, 0.0),
        vfov,
        aspect,
        dist_to_focus,
        aperture,
        t_off, t_span,
        ttl,
    )
}


fn sky(r: &Ray) -> V3 {
    let t: f64 = 0.5 * (r.direction.y / r.direction.length() + 1.0);
    return (1.0 - t) * V3::ones() + t * V3::new(0.5, 0.7, 1.0);
}

fn const_color_dark(_: &Ray) -> V3 { V3::new(0.05088, 0.05088, 0.05088) }
fn const_color_black(_: &Ray) -> V3 { V3::new(0., 0., 0.) }

fn const_color_light(_: &Ray) -> V3 { V3::new(0.3, 0.3, 0.3) }
