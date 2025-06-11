use std::ops::RangeBounds;

use bytes::{Buf, Bytes};

pub trait FlvReader {
    fn available(&self) -> usize;
    fn peek<R: RangeBounds<usize>>(&self, range: R) -> std::io::Result<&[u8]>;

    fn read_u8(&mut self) -> std::io::Result<u8>;
    fn read_i16(&mut self) -> std::io::Result<i16>;
    fn read_u16(&mut self) -> std::io::Result<u16>;
    fn read_u24(&mut self) -> std::io::Result<u32> {
        let mut buff = [0u8; 4];
        self.read_to_slice(&mut buff[1..])?;

        Ok(u32::from_be_bytes(buff))
    }
    fn read_i24(&mut self) -> std::io::Result<i32> {
        let mut buff = [0u8; 4];
        self.read_to_slice(&mut buff[1..])?;

        Ok(i32::from_be_bytes(buff) << 8 >> 8)
    }
    fn read_u32(&mut self) -> std::io::Result<u32>;
    fn read_f64(&mut self) -> std::io::Result<f64>;
    fn read_to_slice(&mut self, buff: &mut [u8]) -> std::io::Result<()>;
    fn read_to_bytes(&mut self, count: usize) -> std::io::Result<Bytes>;
    fn read_to_end(&mut self) -> std::io::Result<Bytes>;
}

impl<T: Buf> FlvReader for T {
    #[inline]
    fn available(&self) -> usize {
        self.remaining()
    }

    #[inline]
    fn peek<R: RangeBounds<usize>>(&self, range: R) -> std::io::Result<&[u8]> {
        Ok(&self.chunk()[(range.start_bound().cloned(), range.end_bound().cloned())])
    }

    #[inline]
    fn read_u8(&mut self) -> std::io::Result<u8> {
        Ok(self.try_get_u8()?)
    }

    #[inline]
    fn read_i16(&mut self) -> std::io::Result<i16> {
        Ok(self.try_get_i16()?)
    }

    #[inline]
    fn read_u16(&mut self) -> std::io::Result<u16> {
        Ok(self.try_get_u16()?)
    }

    #[inline]
    fn read_u32(&mut self) -> std::io::Result<u32> {
        Ok(self.try_get_u32()?)
    }

    #[inline]
    fn read_f64(&mut self) -> std::io::Result<f64> {
        Ok(self.try_get_f64()?)
    }

    #[inline]
    fn read_to_slice(&mut self, buff: &mut [u8]) -> std::io::Result<()> {
        self.try_copy_to_slice(buff)?;
        Ok(())
    }

    #[inline]
    fn read_to_bytes(&mut self, count: usize) -> std::io::Result<Bytes> {
        Ok(self.copy_to_bytes(count))
    }

    #[inline]
    fn read_to_end(&mut self) -> std::io::Result<Bytes> {
        Ok(self.copy_to_bytes(self.remaining()))
    }
}
