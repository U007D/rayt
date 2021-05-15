use crate::{consts::*, Color, Result};
use conv::ValueFrom;
use std::{cmp::max, io::Write, num::NonZeroUsize};

#[derive(Debug)]
pub struct Ppm {
    height:  NonZeroUsize,
    width:   NonZeroUsize,
    x_denom: f64,
    y_denom: f64,
}

impl Ppm {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Result<Self> {
        Ok(Self {
            height,
            width,
            x_denom: f64::value_from(max(1, width.get().saturating_sub(1)))?,
            y_denom: f64::value_from(max(1, height.get().saturating_sub(1)))?,
        })
    }

    pub fn render<ImageWriter: Write, StatusWriter: Write>(
        &self,
        mut image_writer: ImageWriter,
        mut status_writer: StatusWriter,
    ) -> Result<()> {
        self.write_header(&mut image_writer)?;

        (0..self.height.get()).try_for_each::<_, Result<()>>(|row| {
            self.write_status(&mut status_writer, row)?;
            self.write_pixel_row(&mut image_writer, row)
        })?;
        self.flush_status(&mut status_writer)
    }

    #[allow(clippy::unused_self)]
    fn flush_status<ProgressWriter: Write>(
        &self,
        progress_writer: &mut ProgressWriter,
    ) -> Result<()> {
        Ok(writeln!(progress_writer, "\nDone.")?)
    }

    fn write_header<ImageWriter: Write>(&self, image_writer: &mut ImageWriter) -> Result<()> {
        Ok(write!(image_writer, "P3\n{} {}\n255\n", self.width.get(), self.height.get())?)
    }

    fn write_pixel_row<ImageWriter: Write>(
        &self,
        image_writer: &mut ImageWriter,
        row: usize,
    ) -> Result<()> {
        (0..self.width.get()).try_for_each(|col| {
            let pixel = Color::new(
                f64::value_from(col)? / self.x_denom,
                f64::value_from(row)? / self.y_denom,
                0.25,
            );

            pixel.write_u8_color(image_writer)
        })
    }

    fn write_status<ProgressWriter: Write>(
        &self,
        progress_writer: &mut ProgressWriter,
        row: usize,
    ) -> Result<()> {
        let percent_progress = f32::value_from(row)? / f32::value_from(self.height.get())? * 100.0;
        Ok(write!(progress_writer, "\r{}: {:.0}%", msg::PROGRESS, percent_progress)?)
    }
}
