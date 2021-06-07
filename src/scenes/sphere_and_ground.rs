use crate::{
    camera::Camera,
    primitives::sphere::Sphere,
    traits::{IImage, IPixel, IPixelExt, IRgbPixel, ITriplet},
    Error, Pixel, Point3, Result, Vec3, World,
};
use rand::{thread_rng, Rng};
use std::{cmp::max, num::NonZeroUsize};

#[allow(dead_code)]
pub fn render<'iter, TImage>(image: &'iter mut TImage, aa_sample_factor: NonZeroUsize) -> Result<()>
where
    TImage: IImage<Pixel = Pixel>,
    <TImage as IImage>::Pixel: IRgbPixel + IPixelExt + IPixel<Value = f64>,
    <TImage as IImage>::IterMut<'iter>: DoubleEndedIterator, {
    // Initialize world
    let world = init();

    // Initialize camera
    let cam = Camera::new();

    // Render (empty) scene
    let mut rng = thread_rng();
    let image_height = image.height().get();
    let image_width = image.width().get();
    let aa_sample_factor = aa_sample_factor.get();
    image.row_iter_mut().rev().enumerate().try_for_each(|(row, pixels)| {
        pixels.iter_mut().enumerate().try_for_each(|(col, pixel)| {
            let acc_color = (0..aa_sample_factor).try_fold(Vec3::from(pixel.rgb()), |acc, _| {
                let u = (<Pixel as IPixelExt>::try_value_from_usize(col)?
                    + <Pixel as IPixelExt>::try_value_from_f64(rng.gen())?)
                    / <Pixel as IPixelExt>::try_value_from_usize(max(image_width.saturating_sub(1), 1))?;
                let v = (<Pixel as IPixelExt>::try_value_from_usize(row)?
                    + <Pixel as IPixelExt>::try_value_from_f64(rng.gen())?)
                    / <Pixel as IPixelExt>::try_value_from_usize(max(image_height.saturating_sub(1), 1))?;
                let ray = cam.ray(&u, &v);
                Ok::<_, Error>(acc + Vec3::from(ray.color(&world)?.rgb()))
            })?;
            let color = acc_color / <Pixel as IPixelExt>::try_value_from_usize(aa_sample_factor)?;
            pixel.set(color.x(), color.y(), color.z())?;
            Ok::<_, Error>(())
        })
    })
}

fn init() -> World {
    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world
}
