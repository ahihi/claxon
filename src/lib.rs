// Claxon -- A FLAC decoding library in Rust
// Copyright (C) 2014-2015 Ruud van Asseldonk
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License, version 3,
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Claxon, a FLAC decoding library.
//!
//! TODO: Add some examples here.

#![warn(missing_docs)]
#![allow(dead_code)] // TODO: Remove for v0.2

use std::io;
use error::{Error, FlacResult};
use frame::{FrameReader};
use input::ReadExt;
use metadata::{MetadataBlock, MetadataBlockReader, StreamInfo};

mod crc;
mod input;
pub mod error;
pub mod frame;
pub mod sample;
pub mod subframe;
pub mod metadata;

#[allow(missing_docs)]
pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i8 { fn zero() -> i8 { 0i8 } }
impl Zero for i16 { fn zero() -> i16 { 0i16 } }
impl Zero for i32 { fn zero() -> i32 { 0i32 } }
impl Zero for i64 { fn zero() -> i64 { 0i64 } }

#[allow(missing_docs)]
pub trait One {
    fn one() -> Self;
}

impl One for i8 { fn one() -> i8 { 1i8 } }
impl One for i16 { fn one() -> i16 { 1i16 } }
impl One for i32 { fn one() -> i32 { 1i32 } }
impl One for i64 { fn one() -> i64 { 1i64 } }

/// A FLAC decoder that can decode the stream from the underlying reader.
///
/// TODO: Is stream a good name? Should it be called reader/decoder?
/// TODO: Add an example.
pub struct FlacStream<'r> {
    streaminfo: StreamInfo,
    metadata_blocks: Vec<MetadataBlock>,
    input: &'r mut (io::Read + 'r)
}

fn read_stream_header<R: io::Read>(input: &mut R) -> FlacResult<()> {
    // A FLAC stream starts with a 32-bit header 'fLaC' (big endian).
    const HEADER: u32 = 0x66_4c_61_43;
    let header = try!(input.read_be_u32());
    if header != HEADER { return Err(Error::InvalidStreamHeader); }
    Ok(())
}

impl<'r> FlacStream<'r> {
    /// Constructs a flac stream from the given input.
    ///
    /// This will read all metadata and stop at the first audio frame.
    pub fn new<R>(input: &mut R) -> FlacResult<FlacStream> where R: io::Read {
        // A flac stream first of all starts with a stream header.
        try!(read_stream_header(input));

        // Start a new scope, because the input reader must be available again
        // for the frame reader next.
        let (streaminfo, metadata_blocks) = {
            // Next are one or more metadata blocks. The flac specification
            // dictates that the streaminfo block is the first block. The metadata
            // block reader will yield at least one element, so the unwrap is safe.
            let mut metadata_iter = MetadataBlockReader::new(input);
            let streaminfo_block = try!(metadata_iter.next().unwrap());
            let streaminfo = match streaminfo_block {
                MetadataBlock::StreamInfo(info) => info,
                _ => return Err(Error::MissingStreamInfoBlock)
            };

            // There might be more metadata blocks, read and store them.
            let mut metadata_blocks = Vec::new();
            for block_result in metadata_iter {
                match block_result {
                    Err(error) => return Err(error),
                    Ok(block) => metadata_blocks.push(block)
                }
            }

            (streaminfo, metadata_blocks)
        };

        // The flac stream will contain the reader that will read frames.
        let flac_stream = FlacStream {
            streaminfo: streaminfo,
            metadata_blocks: metadata_blocks,
            input: input
        };

        Ok(flac_stream)
    }

    /// Returns the streaminfo metadata.
    pub fn streaminfo(&self) -> &StreamInfo {
        &self.streaminfo
    }

    /// Returns an iterator that decodes a single frame on every iteration.
    pub fn blocks<S: sample::Sample>(&'r mut self) -> FrameReader<'r, S> {
        FrameReader::new(&mut self.input)
    }
}
