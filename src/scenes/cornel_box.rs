use nalgebra::{Isometry3, Matrix, Rotation3, Similarity3, UnitQuaternion};
use crate::consts::PI;
use crate::hittable::{AABoxMono, FlipNormals, IsometryOp, Positionable};
use crate::hittable::IsometryT;
use super::*;

fn cornel_box_prototype() -> (HittableList<Box<dyn Hittable>>, impl Hittable + Clone) {
    let mut result: HittableList<Box<dyn Hittable>> = HittableList::empty();
    let red = Lambertian::<Color>::new(Color::new(0.65, 0.05, 0.05));
    let floor_white = Lambertian::<Color>::new(Color::new(0.73, 0.73, 0.73));
    let ceil_white = Lambertian::<Color>::new(Color::new(0.73, 0.73, 0.73));
    let back_white = Lambertian::<Color>::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::<Color>::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Color::new(1.0, 1.0, 1.0), 15.0);

    result.push(Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 555.0, green).flip_normals()));
    result.push(Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 0.0, red)));
    // let lamp = Box::new(XZRect::new(213.0..343.0, 227.0..332.0, 554.0, light)
    let lamp = XZRect::new(-65.0..65.0, -52.5..52.5, 0.0, light)
        .flip_normals()
        .rotate_y(180.0)
        .translate(V3::new(278.0, 554.0, 279.5));
    let lamp = Box::new(lamp.clone());
    result.push(lamp.clone());
    result.push(Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 0.0, floor_white)));
    result.push(Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 555.0, ceil_white).flip_normals()));
    result.push(Box::new(XYRect::new(0.0..555.0, 0.0..555.0, 555.0, back_white).flip_normals()));
    (result, lamp)
}


pub fn cornel_box_is_reflection(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let white = Lambertian::<Color>::new(Color::new(0.73, 0.73, 0.73));
    let r_light = DiffuseLight::new(Color::new(1.0, 0.0, 0.0), 18.0);
    let g_light = DiffuseLight::new(Color::new(0.0, 1.0, 0.0), 18.0);
    let b_light = DiffuseLight::new(Color::new(0.0, 0.0, 1.0), 18.0);
    let mirror = Metal::new(Color::new(1.0, 1.0, 1.0));
    let mut objs: Vec<Box<dyn Hittable>> = vec![
        /* left wall */
        Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 555.0, white.clone()).flip_normals()),
        /* right wall */
        Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 0.0, white.clone())),
        /* floor */
        Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 0.0, white.clone())),
        /* ceil */
        Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 555.0, white).flip_normals()),
    ];
    let back = Box::new(XYRect::new(0.0..355.0, 0.0..555.0, 555.0, mirror).flip_normals());

    let light1 = Box::new(
        YZRect::new(213.0..343.0, 452.0..532.0, 554.0, b_light)
            .flip_normals()
    );
    let light2 = Box::new(
        YZRect::new(213.0..343.0, 452.0..532.0, 554.0, g_light)
            .flip_normals()
            .translate(V3::new(0.0, 0.0, -100.0))
    );
    let light3 = Box::new(
        YZRect::new(213.0..343.0, 452.0..532.0, 554.0, r_light)
            .flip_normals()
            .translate(V3::new(0.0, 0.0, -200.0))
    );
    objs.push(back.clone());
    objs.push(light1.clone());
    objs.push(light2.clone());
    objs.push(light3.clone());
    let objects: Vec<Box<dyn Hittable>> = vec![light1, light2, light3];
    Scene {
        view: cornel_box_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(HittableList::new(objs)),
            Box::new(HittableList::new(objects)),
            self::const_color_black,
            params,
        ),
    }
}


pub fn cornel_box_with_instances(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let (mut objs, lamp) = cornel_box_prototype();
    let mut important: HittableList<Box<dyn Hittable>> = HittableList::empty();
    let whitebox = AABox::mono(-62.5..62.5, -62.5..62.5, -62.5..62.5,
                               Lambertian::<Color>::new(Color::from_element(0.73)))
        .apply_rotation(Rotation3::from_scaled_axis(V3::y() + V3::x()))
        .translate(V3::new(212.5, 112.5, 147.5))
        // .apply(Isometry3::new(V3::new(212.5, 112.5, 147.5),
        //                       V3::y() + V3::x(),
        /*(UnitQuaternion::from_scaled_axis(V3::z() * (PI / 4.0))
            * UnitQuaternion::from_scaled_axis(V3::y() * (PI / 4.0))
            * UnitQuaternion::from_scaled_axis(V3::x() * (PI / 4.0))).scaled_axis()*/
        // ))
        ;
    // objs.push(Box::new(whitebox.debug_aabb(Color::new(0.8, 0.8, 1.0))));
    objs.push(
        Box::new(whitebox
                 // .rotate_y(-18.0)
                 // .translate(V3::new(130.0, 0.0, 65.0))
        ));


    let shiny_box = Box::new(
        AABox::mono(0.0..165.0, 0.0..330.0, 0.0..165.0, Metal::new(Color::from_element(0.8)))
            // .apply(Isometry3::new(
            //     V3::new(265.0, 0.0, 295.0),
            //     V3::y() * (45.0 / 180.0),
            // ))
        .rotate_y(15.0)
        .translate(V3::new(265.0, 0.0, 295.0))
    );
    objs.push(shiny_box.clone());
    important.push(Box::new(lamp.clone()));


    Scene {
        view: cornel_box_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(objs),
            Box::new(important),
            self::const_color_black,
            params,
        ),
    }
}

pub fn cornel_box_with_is(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let (mut objs, lamp) = cornel_box_prototype();
    let mut important: HittableList<Box<dyn Hittable>> = HittableList::empty();

    let fog = Box::new(ConstantMedium::new(
        Box::new(AABox::mono(-82.5..82.5, -82.5..82.5, -82.5..82.5, NoMat)
                     // .rotate_y(30.0)
                     // .translate(V3::new(212.5, 82.5, 147.5))
                     .apply(Isometry3::new(
                         V3::new(157.5, 82.5, 207.5),
                         V3::y() * Geometry::to_radians(-18.0),
                     ))
                 // .translate(V3::new(130.0, 0.0, 65.0))
                 // .rotate_y(18.0)
        ),
        0.01,
        Color::new(1.0, 1.0, 1.0))
    );

    let lamb = Lambertian::<Color>::new(Color::from_element(0.73));
    let metal = Metal::new(Color::from_element(0.8));
    let shiny_box =
        AABox::new(0.0..165.0, 0.0..330.0, 0.0..165.0,
                   // AABox::new(265.0..(165.0+265.0), 0.0..330.0, 295.0..(165.0+265.0),
                   lamb.clone(),
                   metal.clone(),
                   lamb.clone(),
                   lamb.clone(),
                   lamb.clone(),
                   // lamb.clone(),
                   lamb.clone(),
        )
            .apply(Isometry3::new(
                V3::new(265.0, 0.0, 295.0),
                V3::y() * Geometry::to_radians(15.0)))
        // .rotate_y(-90.0)
        // .translate(V3::new(265.0, 80.0, 295.0))
        ;

    let sphere: Box<Sphere<Dielectric>> = Box::new(
        Sphere::radius(88.5, Dielectric::new(1.5))
            .moved_by(&V3::new(-87.5, 88.5, -12.5))
            .moved_by(&V3::new(130.0, 0.0, 65.0))
            .moved_by(&V3::new(165.0, 165.0, 165.0))
    );

    objs.push(fog.clone());
    objs.push(Box::new(shiny_box.clone()));
    objs.push(sphere.clone());

    // important.push(Box::new(shiny_box.clone()));
    // important.push(fog);

    // important.push(sphere);
    important.push(Box::new(lamp));
    let important = Box::new(important);

    Scene {
        view: cornel_box_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(objs),
            important,
            self::const_color_black,
            params,
        ),
    }
}

pub fn cornel_box_volumes(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let (mut objs, lamp) = cornel_box_prototype();
    objs.push(Box::new(ConstantMedium::new(
        AABox::mono(0.0..165.0, 0.0..165.0, 0.0..165.0, NoMat)
            .rotate_y(-18.0)
            .translate(V3::new(130.0, 0.0, 65.0)),
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    objs.push(Box::new(ConstantMedium::new(
        AABox::mono(0.0..165.0, 0.0..330.0, 0.0..165.0, NoMat)
            .rotate_y(15.0)
            .translate(V3::new(265.0, 0.0, 295.0)),
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    Scene {
        view: cornel_box_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(objs),
            Box::new(lamp),
            self::const_color_dark,
            params,
        ),
    }
}

pub fn cornel_box_test(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let (mut objs, lamp) = cornel_box_prototype();
    let white_fog = ConstantMedium::new(
        AABox::mono(-82.5..82.5, -82.5..82.5, -82.5..82.5, NoMat)
            .rotate_y(30.0)
            .translate(V3::new(212.5, 82.5, 147.5)),
        0.01,
        Color::new(1.0, 1.0, 1.0),
    );

    let white_fog_debug: AABoxMono<_> = (
        Dielectric::new_colored(Color::new(0.5, 0.5, 1.0), 1.0),
        white_fog.bounding_box(0.0..1.0).unwrap()).into();

    objs.push(Box::new(white_fog));
    objs.push(Box::new(white_fog_debug));

    objs.push(Box::new(ConstantMedium::new(
        AABox::mono(0.0..165.0, 0.0..330.0, 0.0..165.0, NoMat)
            .apply(Isometry3::new(
                V3::new(265.0, 0.0, 295.0),
                &V3::y() * (15 as Geometry).to_radians(),
            )),
        // .rotate_y(15.0)
        // .translate(V3::new(265.0, 0.0, 295.0)),
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    Scene {
        view: cornel_box_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(objs),
            Box::new(lamp),
            self::const_color_dark,
            params,
        ),
    }
}


pub fn cornel_box_perlin(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let (mut objs, lamp) = cornel_box_prototype();

    let coord: Normal<Geometry> = rand_distr::Normal::new(0.5, 0.2).unwrap();
    let radius: Normal<Geometry> = rand_distr::Normal::new(1.3, 0.3).unwrap();
    let density: Normal<Scale> = rand_distr::Normal::new(0.5, 0.3).unwrap();
    let sizes = V3::new(100.0, 160.0, 100.0);
    let cloud = (0..1024)
        .map(|_| (random::next(radius), random::next(density), V3::new(
            random::next(coord),
            random::next(coord),
            random::next(coord))))
        .map(|(radius, density, center)| {
            Box::new(ConstantMedium::new(
                Sphere::new(center.component_mul(&sizes).into(), radius.abs() * 10.0 + 10.0, Lambertian::default()),
                density.abs() * 0.02,
                Color::new(0.7, 0.7, 0.5),
            )) as Box<dyn Hittable>
        })
        .collect_vec();
    let cloud_bvh = BVH::new(cloud)
        // .rotate_y(15.0)
        .translate(V3::new(200.0, 200.0, 200.0));
    objs.push(Box::new(cloud_bvh));

    Scene {
        view: cornel_box_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(objs),
            Box::new(lamp),
            self::const_color_dark,
            params,
        ),
    }
}


fn cornel_box_cam(nx: u32, ny: u32, t_off: Time, t_span: Time, ttl: i32) -> View {
    let aspect = (nx as Geometry) / (ny as Geometry);
    let from = V3::new(278.0, 278.0, -680.0);
    let at = V3::new(278.0, 278.0, 0.0);

    let dist_to_focus = 2.0;
    let aperture = 0.00;
    let vfov = 80.0;
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
