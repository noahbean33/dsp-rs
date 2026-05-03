use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const BMP_HEADER_SIZE: usize = 54;
const BMP_COLOR_TABLE_SIZE: usize = 1024;

/// A BMP image with header, optional color table, and pixel data.
#[derive(Debug, Clone)]
pub struct BmpImage {
    pub width: i32,
    pub height: i32,
    pub bit_depth: i32,
    pub header: [u8; BMP_HEADER_SIZE],
    pub color_table: [u8; BMP_COLOR_TABLE_SIZE],
    pub data: Vec<u8>,
}

impl BmpImage {
    /// Create an empty `BmpImage` with zeroed header and no pixel data.
    #[must_use]
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            bit_depth: 0,
            header: [0u8; BMP_HEADER_SIZE],
            color_table: [0u8; BMP_COLOR_TABLE_SIZE],
            data: Vec::new(),
        }
    }

    /// Read a BMP file from disk.
    ///
    /// Supports 8-bit grayscale (with color table) and 24-bit RGB images.
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let mut file =
            File::open(path.as_ref()).map_err(|e| format!("Unable to open file: {e}"))?;

        let mut header = [0u8; BMP_HEADER_SIZE];
        file.read_exact(&mut header)
            .map_err(|e| format!("Unable to read header: {e}"))?;

        let width = i32::from_le_bytes([header[18], header[19], header[20], header[21]]);
        let height = i32::from_le_bytes([header[22], header[23], header[24], header[25]]);
        let bit_depth = i32::from_le_bytes([header[28], header[29], header[30], header[31]]);

        let mut color_table = [0u8; BMP_COLOR_TABLE_SIZE];
        if bit_depth <= 8 {
            file.read_exact(&mut color_table)
                .map_err(|e| format!("Unable to read color table: {e}"))?;
        }

        let data_size = (width.unsigned_abs() as usize) * (height.unsigned_abs() as usize)
            * if bit_depth > 8 {
                (bit_depth as usize) / 8
            } else {
                1
            };
        let mut data = vec![0u8; data_size];
        let bytes_read = file.read(&mut data).map_err(|e| format!("Unable to read data: {e}"))?;
        data.truncate(bytes_read);

        Ok(Self {
            width,
            height,
            bit_depth,
            header,
            color_table,
            data,
        })
    }

    /// Write the BMP image to a file on disk.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let mut file =
            File::create(path.as_ref()).map_err(|e| format!("Unable to create file: {e}"))?;

        file.write_all(&self.header)
            .map_err(|e| format!("Unable to write header: {e}"))?;

        if self.bit_depth <= 8 {
            file.write_all(&self.color_table)
                .map_err(|e| format!("Unable to write color table: {e}"))?;
        }

        file.write_all(&self.data)
            .map_err(|e| format!("Unable to write data: {e}"))?;

        Ok(())
    }

    /// Create a deep copy of this image with cloned pixel data.
    #[must_use]
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// Return the total number of pixels (width × height).
    #[must_use]
    pub fn pixel_count(&self) -> usize {
        (self.width.unsigned_abs() as usize) * (self.height.unsigned_abs() as usize)
    }

    /// Create a new image that shares the same header and color table but with
    /// the given pixel data.
    #[must_use]
    pub fn with_data(&self, data: Vec<u8>) -> Self {
        Self {
            width: self.width,
            height: self.height,
            bit_depth: self.bit_depth,
            header: self.header,
            color_table: self.color_table,
            data,
        }
    }
}

impl Default for BmpImage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_image_has_zero_dimensions() {
        let img = BmpImage::new();
        assert_eq!(img.width, 0);
        assert_eq!(img.height, 0);
        assert_eq!(img.bit_depth, 0);
        assert!(img.data.is_empty());
    }

    #[test]
    fn copy_produces_independent_clone() {
        let mut img = BmpImage::new();
        img.data = vec![1, 2, 3];
        let copy = img.copy();
        assert_eq!(copy.data, vec![1, 2, 3]);
    }
}
