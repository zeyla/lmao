//! Efficiently decompress Discord gateway messages.
//!
//! The [`Inflater`] decompresses messages sent over the gateway by reusing a
//! common buffer to minimize the amount of allocations in the hot path.

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use zstd_safe::{DCtx, InBuffer, OutBuffer, ResetDirective};

/// An operation relating to compression failed.
#[derive(Debug)]
pub struct CompressionError {
    /// Type of error.
    kind: CompressionErrorType,
    /// Source error if available.
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CompressionError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CompressionErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (CompressionErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }

    /// Shortcut to create a new error for an erroneous status code.
    fn from_code(code: usize) -> Self {
        Self {
            kind: CompressionErrorType::Decompressing,
            source: Some(zstd_safe::get_error_name(code).into()),
        }
    }
}

impl Display for CompressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            CompressionErrorType::Decompressing => f.write_str("message could not be decompressed"),
            CompressionErrorType::NotUtf8 => f.write_str("decompressed message is not UTF-8"),
        }
    }
}

impl Error for CompressionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`CompressionError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CompressionErrorType {
    /// Decompressing a frame failed.
    Decompressing,
    /// Decompressed message is not UTF-8.
    NotUtf8,
}

/// Gateway event decompressor.
///
/// Each received compressed event gets inflated into a [`String`] who's input
/// and output size is recorded.
///
/// # Example
///
/// Calculate the percentage bytes saved:
/// ```
/// # use twilight_gateway::{Intents, Shard, ShardId};
/// # #[tokio::main] async fn main() {
/// # let shard = Shard::new(ShardId::ONE, String::new(), Intents::empty());
/// let inflater = shard.inflater();
/// let total_percentage_compressed =
///     inflater.processed() as f64 * 100.0 / inflater.produced() as f64;
/// let total_percentage_saved = 100.0 - total_percentage_compressed;
/// # }
/// ```
pub struct Inflater {
    /// Common decompressed message buffer.
    buffer: Vec<u8>,
    /// Reusable zstd decompression context.
    ctx: DCtx<'static>,
    /// Total number of bytes processed.
    processed: u64,
    /// Total number of bytes produced.
    produced: u64,
}

impl Debug for Inflater {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Inflater")
            .field("buffer", &self.buffer)
            .field("ctx", &"<zstd decompression context>")
            .field("processed", &self.processed)
            .field("produced", &self.produced)
            .finish()
    }
}

impl Inflater {
    /// Create a new inflator for a shard.
    pub(crate) fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(DCtx::out_size()),
            ctx: DCtx::create(),
            processed: 0,
            produced: 0,
        }
    }

    /// Decompress message.
    ///
    /// # Errors
    ///
    /// Returns a [`CompressionErrorType::Decompressing`] error type if the
    /// message could not be decompressed.
    ///
    /// Returns a [`CompressionErrorType::NotUtf8`] error type if the
    /// decompressed message is not UTF-8.
    pub(crate) fn inflate(&mut self, message: &[u8]) -> Result<String, CompressionError> {
        let mut input = InBuffer::around(message);

        // Decompressed message. `Vec::extend_from_slice` efficiently allocates
        // only what's necessary.
        let mut decompressed = Vec::new();

        loop {
            let mut output = OutBuffer::around(&mut self.buffer);

            self.ctx
                .decompress_stream(&mut output, &mut input)
                .map_err(CompressionError::from_code)?;

            decompressed.extend_from_slice(output.as_slice());

            // Break when message has been fully decompressed.
            if input.pos == input.src.len() && output.pos() != output.capacity() {
                break;
            }
        }

        self.processed += u64::try_from(input.src.len()).unwrap();
        self.produced += u64::try_from(decompressed.len()).unwrap();

        String::from_utf8(decompressed).map_err(|source| CompressionError {
            kind: CompressionErrorType::NotUtf8,
            source: Some(Box::new(source)),
        })
    }

    /// Reset the inflater's state.
    pub(crate) fn reset(&mut self) {
        self.ctx
            .reset(ResetDirective::SessionOnly)
            .expect("resetting session is infallible");
    }

    /// Total number of bytes processed.
    pub const fn processed(&self) -> u64 {
        self.processed
    }

    /// Total number of bytes produced.
    pub const fn produced(&self) -> u64 {
        self.produced
    }
}
