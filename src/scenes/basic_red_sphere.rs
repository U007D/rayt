use crate::{
    consts::*,
    primitives::{ray::Ray, sphere::Sphere},
    traits::{IImage, IPixel, IPixelExt, IRgbPixel},
    world::World,
    Error, Pixel, Point3, Result, Vec3,
};
use std::cmp::max;

#[allow(dead_code)]
pub fn render<TImage>(image: &mut TImage) -> Result<()>
where
    TImage: IImage<Pixel = Pixel>,
    <TImage as IImage>::Pixel: IRgbPixel + IPixelExt + IPixel<Value = f64>, {
    // Initialize world
    let world = init();

    // Set up camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let origin = Point3::default();
    let x_axial = Vec3::new(viewport_width, 0.0, 0.0);
    let y_axial = Vec3::new(0.0, viewport_height, 0.0);
    let z_axial = Vec3::new(0.0, 0.0, focal_length);
    let lower_left_corner = origin - x_axial / 2.0 - y_axial / 2.0 - z_axial;

    // Render (empty) scene
    let image_height = image.height().get();
    let image_width = image.width().get();
    image.row_iter_mut().enumerate().try_for_each(|(row, pixels)| {
        pixels.iter_mut().enumerate().try_for_each(|(col, pixel)| {
            let u = <Pixel as IPixelExt>::try_value_from_usize(col)?
                / <Pixel as IPixelExt>::try_value_from_usize(max(image_width.saturating_sub(1), 1))?;
            let v = <Pixel as IPixelExt>::try_value_from_usize(row)?
                / <Pixel as IPixelExt>::try_value_from_usize(max(image_height.saturating_sub(1), 1))?;
            let ray = Ray::new(origin, lower_left_corner + u * x_axial + v * y_axial - origin);
            pixel.set_pixel(ray.color(&world)?);
            Ok::<_, Error>(())
        })
    })
}

fn init() -> World {
    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world
}
