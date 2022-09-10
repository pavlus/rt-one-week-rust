use nalgebra::{Isometry3, Rotation3};
use rand::Rng;
use crate::bvh::{BoundedHittable, BVHIndexed};
use crate::hittable::{AABoxMono, Bounded, Important, IsometryOp, Positionable};
use crate::material::PolishedMetal;
use super::*;

type CornelBoxScene = SceneDesc<Vec<Box<dyn BoundedHittable>>, Lamp>;
type Lamp = XZRect<DiffuseLight<Color>>;

fn cornel_box_prototype() -> (Vec<Box<dyn BoundedHittable>>, Lamp) {
    let mut result: Vec<Box<dyn BoundedHittable>> = Vec::new();
    let red = Lambertian::<Color>::new(Color::new(0.65, 0.05, 0.05));
    let floor_white = Lambertian::<Color>::new(Color::new(0.73, 0.73, 0.73));
    let ceil_white = Lambertian::<Color>::new(Color::new(0.73, 0.73, 0.73));
    let back_white = Lambertian::<Color>::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::<Color>::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Color::new(1.0, 1.0, 1.0), 15.0);

    result.push(Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 555.0, green).flip_normals()));
    result.push(Box::new(YZRect::new(0.0..555.0, 0.0..555.0, 0.0, red)));
    let lamp: XZRect<DiffuseLight<Color>> = XZRect::new(213.0..343.0, 227.0..332.0, 554.0, light);
    result.push(Box::new(lamp.clone().flip_normals().clone()));
    result.push(Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 0.0, floor_white)));
    result.push(Box::new(XZRect::new(0.0..555.0, 0.0..555.0, 555.0, ceil_white).flip_normals()));
    result.push(Box::new(XYRect::new(0.0..555.0, 0.0..555.0, 555.0, back_white).flip_normals()));
    (result, lamp)
}


pub fn cornel_box_is_reflection(timespan: Timespan, params: &Params) -> SceneDesc<Vec<Box<dyn BoundedHittable>>, Vec<YZRect<DiffuseLight<Color>>>> {
    let white = Lambertian::<Color>::new(Color::new(0.73, 0.73, 0.73));
    let r_light = DiffuseLight::new(Color::new(1.0, 0.0, 0.0), 18.0);
    let g_light = DiffuseLight::new(Color::new(0.0, 1.0, 0.0), 18.0);
    let b_light = DiffuseLight::new(Color::new(0.0, 0.0, 1.0), 18.0);
    let mirror = PolishedMetal::new(Color::new(1.0, 1.0, 1.0));
    let mut objs: Vec<Box<dyn BoundedHittable>> = vec![
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

    let light1 = YZRect::new(213.0..343.0, 452.0..532.0, 554.0, b_light);
    let light2 = YZRect::new(213.0..343.0, 452.0..532.0, 554.0, g_light)
        .moved_by(&V3::new(0.0, 0.0, -100.0));
    let light3 = YZRect::new(213.0..343.0, 452.0..532.0, 554.0, r_light)
        .moved_by(&V3::new(0.0, 0.0, -200.0));
    objs.push(back.clone());
    objs.push(Box::new(light1.clone().flip_normals()));
    objs.push(Box::new(light2.clone().flip_normals()));
    objs.push(Box::new(light3.clone().flip_normals()));
    let objects: Vec<YZRect<DiffuseLight<Color>>> = vec![light1, light2, light3];
    SceneDesc {
        view: cornel_box_view(params.width, params.height, timespan, params.bounces as i32),
        hittable: objs,
        important: objects,
        miss_shader: const_color_black,
    }
}


pub fn cornel_box_with_instances(timespan: Timespan, params: &Params) -> SceneDesc<Vec<Box<dyn BoundedHittable>>, Lamp> {
    let (mut objs, lamp) = cornel_box_prototype();
    let mut important: Vec<Box<dyn Important>> = Vec::new();
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
        AABox::mono(0.0..165.0, 0.0..330.0, 0.0..165.0, PolishedMetal::new(Color::from_element(0.8)))
            // .apply(Isometry3::new(
            //     V3::new(265.0, 0.0, 295.0),
            //     V3::y() * (45.0 / 180.0),
            // ))
            .rotate_y(15.0)
            .translate(V3::new(265.0, 0.0, 295.0))
    );
    objs.push(shiny_box.clone());
    important.push(Box::new(lamp.clone()));


    SceneDesc {
        view: cornel_box_view(params.width, params.height, timespan, params.bounces as i32),
        hittable: objs,
        important: lamp,
        miss_shader: const_color_black,
    }
}

pub fn cornel_box_with_is(timespan: Timespan, params: &Params) -> SceneDesc<BVHIndexed<Box<dyn BoundedHittable>>, Lamp> {
    let (mut objs, lamp) = cornel_box_prototype();

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
    let metal = PolishedMetal::new(Color::from_element(0.8));
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
    // important.push(Box::new(lamp));

    let graph = BVHIndexed::new(objs, timespan.clone());
    // eprintln!("{:?}", graph);
    SceneDesc {
        view: cornel_box_view(params.width, params.height, timespan, params.bounces as i32),
        hittable: graph,
        important: lamp,
        miss_shader: const_color_black,
    }
}

pub fn cornel_box_volumes(timespan: Timespan, params: &Params) -> SceneDesc<Vec<Box<dyn BoundedHittable>>, Lamp> {
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

    SceneDesc {
        view: cornel_box_view(params.width, params.height, timespan, params.bounces as i32),
        hittable: objs,
        important: lamp,
        miss_shader: const_color_dark,
    }
}

pub fn cornel_box_test(timespan: Timespan, params: &Params) -> CornelBoxScene {
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
        white_fog.bounding_box(0.0..1.0)).into();

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

    SceneDesc {
        view: cornel_box_view(params.width, params.height, timespan, params.bounces as i32),
        hittable: objs,
        important: lamp,
        miss_shader: const_color_dark,
    }
}


pub fn cornel_box_perlin(timespan: Timespan, params: &Params) -> CornelBoxScene {
    let (mut objs, lamp) = cornel_box_prototype();
    let mut rng = DefaultRng::default();
    let coord: Normal<Geometry> = Normal::new(0.5, 0.2).unwrap();
    let radius: Normal<Geometry> = Normal::new(1.3, 0.3).unwrap();
    let density: Normal<Scale> = Normal::new(0.5, 0.3).unwrap();
    let sizes = V3::new(100.0, 160.0, 100.0);
    let cloud = (0..1024)
        .map(|_| (rng.sample(radius), rng.sample(density), V3::new(
            rng.sample(coord),
            rng.sample(coord),
            rng.sample(coord))))
        .map(|(radius, density, center)| {
            Box::new(ConstantMedium::new(
                Sphere::new(center.component_mul(&sizes).into(), radius.abs() * 10.0 + 10.0, Lambertian::default()),
                density.abs() * 0.02,
                Color::new(0.7, 0.7, 0.5),
            ))
        })
        .collect_vec();
    let cloud_bvh = BVHIndexed::new(cloud, timespan.clone())
        // .rotate_y(15.0)
        .translate(V3::new(200.0, 200.0, 200.0));
    objs.push(Box::new(cloud_bvh));

    SceneDesc {
        view: cornel_box_view(params.width, params.height, timespan, params.bounces as i32),
        hittable: objs,
        important: lamp,
        miss_shader: const_color_dark,
    }
}


fn cornel_box_view(nx: u32, ny: u32, timespan: Timespan, ttl: i32) -> View {
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
        timespan,
        ttl,
    )
}
