use crate::Result;
use std::io::Write;

pub trait IEncoder<T>
where
    T: ?Sized, {
    fn encode<TOutputDevice>(source: &T, output_device: &mut TOutputDevice) -> Result<()>
    where
        TOutputDevice: Write;
}

pub trait IEncoderProgress<T>: IEncoder<T>
where
    T: ?Sized, {
    fn encode<TOutputDevice, TStatusDevice>(
        source: &T,
        output_device: &mut TOutputDevice,
        status_device: &mut TStatusDevice,
    ) -> Result<()>
    where
        TOutputDevice: Write,
        TStatusDevice: Write;
}
