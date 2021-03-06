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

//! The `error` module defines the error and result types.

use std::io;

/// An error that prevents succesful decoding of the FLAC stream.
#[derive(Debug)]
pub enum Error {
    /// Not a decoding error, but a problem with the underlying IO.
    IoError(io::Error),

    /// The stream header does not equal 'fLaC'.
    InvalidStreamHeader,

    /// Metadata block type 127 is invalid, to avoid confusion with a frame sync code.
    InvalidMetadataBlockType,
    /// The streaminfo block must have length 34.
    InvalidMetadataBlockLength,

    /// A lower bound was encountered that was bigger than an upper bound.
    InconsistentBounds,
    /// The minimum block size must be larger than 15, and the block size must
    /// not exceed 65535.
    InvalidBlockSize,
    /// The sample rate must be positive and no larger than 6553550 Hz.
    InvalidSampleRate,

    /// The streaminfo block must be the very first metadata block.
    MissingStreamInfoBlock,

    /// A frame must start with the frame sync code.
    MissingFrameSyncCode,
    /// The frame header contains an invalid value in one of the reserved bits,
    /// or it contains one of the bit patterns that is invalid to prevent
    /// confusion with a frame sync code, or a bit pattern that is reserved.
    InvalidFrameHeader,
    /// The expected UTF-8-ish encoded integer contains invalid bit sequences.
    InvalidVarLengthInt,
    /// The observed frame header CRC does not match the stored CRC.
    FrameHeaderCrcMismatch,

    /// The subframe header contains an invalid or reserved bit pattern.
    InvalidSubframeHeader,
    /// The subframe contains an invalid or reserved bit pattern.
    InvalidSubframe,

    /// The residual contains an invalid or reserved bit pattern.
    InvalidResidual,
    /// The number of bits per sample in an unencoded binary Rice partition
    /// is larger than the bits per sample of the stream.
    InvalidBitsPerSample,
    /// A bit pattern is not a valid Rice code in the context.
    InvalidRiceCode,
    /// An overflow occurred when decoding the side channel.
    InvalidSideSample,
    /// An overflow occurred when doing fixed prediction.
    InvalidFixedSample,
    /// An overflow occurred when doing LPC prediction.
    InvalidLpcSample,

    /// The audio stream has more bits per sample than the provided sample
    /// buffer to decode into.
    SampleTooWide
}

// TODO: implement the Error trait for claxon::error::Error.

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        use error::Error::{InvalidStreamHeader,
            InvalidMetadataBlockType,
            InvalidMetadataBlockLength,
            // ...
            InvalidVarLengthInt
        };
        match (self, other) {
            (&InvalidStreamHeader, &InvalidStreamHeader) => true,
            (&InvalidMetadataBlockType, &InvalidMetadataBlockType) => true,
            (&InvalidMetadataBlockLength, &InvalidMetadataBlockLength) => true,
            (&InvalidVarLengthInt, &InvalidVarLengthInt) => true,
            // TODO: this is not complete, yet.
            // TODO: this is both cumbersome and error-prone. The _ case is
            // required for all non-equal combinations, but it will prevent the
            // compiler from emitting a warning once a new enum variant is
            // added. There must be a better way, right?
            _ => false
        }
    }
}

/// Either `T` on success, or an `Error` on failure.
pub type FlacResult<T> = Result<T, Error>;
