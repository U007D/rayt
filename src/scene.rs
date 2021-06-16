use crate::{
    camera::Camera,
    consts::*,
    primitives::{Pixel, Point3, Sphere, Vec3},
    traits::{IImage, IPixel, IPixelExt, IRender, IRgbPixel, ITriplet},
    world::World,
    Error, Result,
};
use conv::ValueFrom;

use crate::traits::IRenderProgress;
use rand::{thread_rng, Rng};
use std::{
    cmp::max,
    io::{sink, Write},
    num::NonZeroUsize,
};

#[derive(Debug)]
pub struct Scene {}

impl Scene {
    #[must_use]
    pub const fn new() -> Self { Self {} }

    #[must_use]
    fn init() -> World {
        let mut world = World::new();
        world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
        world
    }

    fn write_status<TStatusDevice>(current: usize, max_value: usize, status_device: &mut TStatusDevice) -> Result<()>
    where
        TStatusDevice: Write, {
        let percent_progress =
            f32::value_from(current)? / f32::value_from(max(max_value.saturating_sub(1), 1))? * 100.0;
        write!(status_device, "\r{}: {:.0}%", msg::RENDER_PROGRESS, percent_progress)?;
        Ok(())
    }
}

impl IRender for Scene {
    type Pixel = Pixel;
    type Value = f64;

    fn render<'iter, TImage>(&self, image: &'iter mut TImage, aa_sample_factor: NonZeroUsize) -> Result<()>
    where
        TImage: IImage<Pixel = Self::Pixel>,
        <TImage as IImage>::Pixel: IRgbPixel + IPixelExt + IPixel<Value = Self::Value> + 'iter,
        <TImage as IImage>::IterMut<'iter>: DoubleEndedIterator, {
        <Self as IRenderProgress>::render(self, image, aa_sample_factor, &mut sink())
    }
}

impl IRenderProgress for Scene {
    fn render<'iter, TImage, TStatusDevice>(
        &self,
        image: &'iter mut TImage,
        aa_sample_factor: NonZeroUsize,
        mut status_device: TStatusDevice,
    ) -> Result<()>
    where
        TImage: IImage<Pixel = Self::Pixel>,
        <TImage as IImage>::Pixel: IRgbPixel + IPixelExt + IPixel<Value = Self::Value> + 'iter,
        <TImage as IImage>::IterMut<'iter>: DoubleEndedIterator,
        TStatusDevice: Write, {
        // Initialize world
        let world = Self::init();

        // Initialize camera
        let cam = Camera::new();

        // Render (empty) scene
        let mut rng = thread_rng();
        let image_height = image.height().get();
        let image_width = image.width().get();
        let aa_sample_factor = aa_sample_factor.get();
        let row_count = image.height().get();
        image.iter_mut().rev().enumerate().try_for_each(|(row, pixels)| {
            Self::write_status(row, row_count, &mut status_device)?;
            pixels.iter_mut().enumerate().try_for_each(|(col, pixel)| {
                let acc_color = (0..aa_sample_factor).try_fold(Vec3::from(pixel.rgb()), |acc, _| {
                    let u = (<Pixel as IPixelExt>::try_value_from_usize(col)?
                        + <Pixel as IPixelExt>::try_value_from_f64(rng.gen())?)
                        / <Pixel as IPixelExt>::try_value_from_usize(max(image_width.saturating_sub(1), 1))?;
                    let v = (<Pixel as IPixelExt>::try_value_from_usize(row)?
                        + <Pixel as IPixelExt>::try_value_from_f64(rng.gen())?)
                        / <Pixel as IPixelExt>::try_value_from_usize(max(image_height.saturating_sub(1), 1))?;
                    let ray = cam.ray(&u, &v);
                    Ok::<_, Error>(acc + Vec3::from(ray.color(&world, MAX_RENDER_DEPTH)?.rgb()))
                })?;
                let color = acc_color / <Pixel as IPixelExt>::try_value_from_usize(aa_sample_factor)?;
                pixel.set(color.x(), color.y(), color.z())?;
                Ok::<_, Error>(())
            })
        })
    }
}
