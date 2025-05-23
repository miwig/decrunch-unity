// Copyright (c) Istvan Fehervari

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! # Decoder for crunch-compressed texture data
//!
//! This crate provides a Rust wrapper around [crunch's](https://github.com/BinomialLLC/crunch) decompressor.
//!
//! # Example
//!
//! ```
//! use decrunch::*;
//! use std::fs::File;
//! use std::io::Read;
//!
//! # use std::io;
//! # fn foo() -> io::Result<()> {
//! let mut compressed_file = File::open("testdata/copyright_2048_compressed.dat")?;
//! let mut compressed_data = Vec::new();
//!
//! compressed_file.read_to_end(&mut compressed_data)?;
//!
//! let c_data = CrunchedData::new(&compressed_data);
//! let decompressed_data = match c_data.decode_level(0) {
//!     None => {
//!         panic!("Failed to decompress texture data");
//!     }
//!     Some(res) => res,
//! };
//!
//! assert!(decompressed_data.len() > 0);
//!
//! # Ok(())
//! # }
//! ```

extern crate libc;

mod crunch;

use libc::c_void;
use std::mem;

// Rust with repr(C) actually does 8 bytes here, as does the c compiler on linux
// For a windows build (at least with cargo xwin), the c compiler will actually
// use 4 bytes as intended, so we use repr(i32) to match. This is probably
// the wrong way to fix this, but works and doesn't require changing the c code.
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(target_os = "linux", repr(C))]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum CrnFormat {
    FirstValid = -2,
    #[default]
    Invalid = -1,
    Dxt1 = 0,
    /// cCRNFmtDXT3 is not currently supported when writing to CRN - only DDS.
    Dxt3,
    Dxt5,
    // Various DXT5 derivatives
    /// Luma-chroma
    Dxt5cCxY,
    /// Swizzled 2-component
    Dxt5xGxR,
    /// Swizzled 3-component
    Dxt5xGBR,
    /// Swizzled 4-component
    Dxt5Agbr,

    /// ATI 3DC and X360 DXN
    DxNXy,
    DxNYx,

    /// DXT5 alpha blocks only
    Dxt5A,
    Etc1,
    Total,
    #[cfg(target_os = "linux")] // this doesn't work with repr(i32)
    ForceDWORD = 0xFFFFFFFF,
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LevelInfo {
    pub struct_size: u32,
    pub width: u32,
    pub height: u32,
    pub faces: u32,
    pub blocks_x: u32,
    pub blocks_y: u32,
    pub bytes_per_block: u32,
    pub format: CrnFormat,
}

impl Default for LevelInfo {
    fn default() -> LevelInfo {
        LevelInfo {
            struct_size: mem::size_of::<LevelInfo>() as u32,
            width: 0,
            height: 0,
            faces: 0,
            blocks_x: 0,
            blocks_y: 0,
            bytes_per_block: 0,
            format: CrnFormat::Invalid,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct TextureInfo {
    pub struct_size: u32,
    pub width: u32,
    pub height: u32,
    pub levels: u32,
    pub faces: u32,
    pub bytes_per_block: u32,
    pub userdata0: u32,
    pub userdata1: u32,
    pub format: CrnFormat,
}

impl Default for TextureInfo {
    fn default() -> TextureInfo {
        TextureInfo {
            struct_size: mem::size_of::<TextureInfo>() as u32,
            width: 0,
            height: 0,
            levels: 0,
            faces: 0,
            bytes_per_block: 0,
            userdata0: 0,
            userdata1: 0,
            format: CrnFormat::Invalid,
        }
    }
}

pub struct CrunchedData<'a> {
    pub buffer: &'a [u8],
    ctx: *const c_void,
}

impl<'a> CrunchedData<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        CrunchedData {
            buffer,
            ctx: crunch::unpack_begin(buffer),
        }
    }

    /// Retrieves mipmap level specific information from the CRN data.
    pub fn level_info(&self, level: u32) -> LevelInfo {
        crunch::get_level_info(self, level)
    }

    /// Retrieves texture information from the CRN data.
    pub fn texture_info(&self) -> TextureInfo {
        crunch::get_texture_info(self)
    }

    /// Transcodes the specified mipmap level to a destination buffer.
    pub fn decode_level(&self, level: u32) -> Option<Vec<u8>> {
        let info = self.level_info(level);
        let mut dst: Vec<u8> =
            vec![0; (info.blocks_x * info.blocks_y * info.bytes_per_block) as usize];
        if !crunch::unpack_level(
            self.ctx,
            &mut dst,
            info.blocks_x * info.bytes_per_block,
            level,
        ) {
            return None;
        }
        Some(dst)
    }
}

impl Drop for CrunchedData<'_> {
    fn drop(&mut self) {
        crunch::unpack_end(self.ctx);
    }
}

#[cfg(test)]
mod tests;
