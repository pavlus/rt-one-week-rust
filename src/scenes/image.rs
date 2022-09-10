use super::*;

pub fn img_scene(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let mut objs: Vec<Box<dyn Hittable>> = Vec::new();
    objs.push(Box::new(
        Sphere::new(P3::new(0.0, -1000.0, 0.0), 1000.0,
                    Lambertian::texture(Checker::new(
                        Color::new(0.0, 0.0, 0.0),
                        Color::new(1.0, 1.0, 1.0),
                        10.0,
                    )))));
    objs.push(Box::new(Sphere::new(P3::new(0.0, 2.0, 0.0), 2.0,
                                   Lambertian::texture(ImageTexture::load("./textures/stone.png")))));

    Scene {
        camera: get_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(HittableList::new(objs)),
            Box::new(NoHit),
            self::const_color_light,
            params,
        ),
    }
}

pub fn img_lit_scene(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let light = Sphere::new(P3::new(0.0, 3.0, -2.0), 2.0,
                            DiffuseLight::new(Color::new(1.0, 1.0, 0.99), 2.0));
    let light1 = Sphere::new(P3::new(0.0, 3.0, -2.0), 2.0,
                             DiffuseLight::new(Color::new(1.0, 1.0, 0.99), 2.0));
    let objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(P3::new(0.0, -1000.0, 0.0), 1000.0,
                             Lambertian::texture(Checker::new(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0), 10.0)))),
        Box::new(Sphere::new(P3::new(0.0, 2.0, 2.0), 2.0,
                             Lambertian::texture(ImageTexture::load("./textures/stone.png")))),
        Box::new(light),
    ];
    Scene {
        camera: get_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(HittableList::new(objs)),
            Box::new(light1),
            self::const_color_dark,
            params,
        ),
    }
}

#[allow(dead_code)]
pub fn img_lit_rect_scene(t_off: Time, t_span: Time, params: &Params) -> Scene {
    let l1 = Box::new(XZRect::new(
        -1.0..1.0, -1.0..1.0, 2.5,
        DiffuseLight::new(Color::new(1.0, 1.0, 0.99), 4.0)));
    let l2= Box::new(XYRect::new(
        -1.0..1.0, 0.5..1.5, -1.5,
        DiffuseLight::new(Color::new(1.0, 1.0, 0.99), 4.0)));
    let objs: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(P3::new(0.0, -1000.0, 0.0), 1000.0,
                             Lambertian::texture(Checker::new(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0), 10.0)))),
        Box::new(Sphere::new(P3::new(0.0, 1.0, 0.0), 1.0,
                             Lambertian::texture(ImageTexture::load("./textures/stone.png")))),
        l1.clone(),
        l2.clone(),
    ];
    let objects: Vec<Box<dyn Hittable>> = vec![l1, l2];
    Scene {
        camera: get_cam(params.width, params.height, t_off, t_span, params.bounces as i32),
        renderer: RendererImpl::pick_renderer(
            Box::new(HittableList::new(objs)),
            Box::new(HittableList::new(objects)),
            self::const_color_dark,
            params,
        ),
    }
}
