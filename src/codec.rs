//! Implementation of ipld-core's `Codec` trait.

use std::convert::TryFrom;
use std::io::{BufRead, Write};

use ipld_core::{
    cid::Cid,
    codec::{Codec, Links},
    serde::ExtractLinks,
};
use serde::{de::Deserialize, ser::Serialize};

use crate::{de::Deserializer, error::CodecError};

/// DAG-CBOR implementation of ipld-core's `Codec` trait.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DagCborCodec;

const RAW_CODE: u64 = 0x71;

impl<T> Codec<T> for DagCborCodec
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    type Error = CodecError;

    fn to_code(&self) -> u64 {
        RAW_CODE
    }

    fn try_from_code(code: u64) -> Option<Self> {
        match code {
            RAW_CODE => Some(DagCborCodec),
            _ => None
        }
    }

    fn decode<R: BufRead>(reader: R) -> Result<T, Self::Error> {
        Ok(crate::from_reader(reader)?)
    }

    fn encode<W: Write>(writer: W, data: &T) -> Result<(), Self::Error> {
        Ok(crate::to_writer(writer, data)?)
    }
}

impl Links for DagCborCodec {
    type LinksError = CodecError;

    fn links(&self, data: &[u8]) -> Result<impl Iterator<Item = Cid>, Self::LinksError> {
        let mut deserializer = Deserializer::from_slice(data);
        Ok(ExtractLinks::deserialize(&mut deserializer)?
            .into_vec()
            .into_iter())
    }
}

impl From<DagCborCodec> for u64 {
    fn from(_: DagCborCodec) -> u64 {
        RAW_CODE
    }
}

impl TryFrom<u64> for DagCborCodec {
    type Error = NotDagCborCode;

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        if code == RAW_CODE {
            Ok(DagCborCodec)
        } else {
            Err(NotDagCborCode(code))
        }
    }
}

/// FIXME
pub struct NotDagCborCode(pub u64);
