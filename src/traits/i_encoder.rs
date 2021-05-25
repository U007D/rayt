use crate::Result;
use std::io::Write;

pub trait IEncoder<T>
where
    T: ?Sized, {
    fn encode<TOutputDevice: Write>(output_device: &mut TOutputDevice, source: &T) -> Result<()>;
}
