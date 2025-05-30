//! CRAM container compression header.

pub mod data_series_encodings;
pub(crate) mod encoding;
pub mod preservation_map;
mod tag_encodings;

pub(crate) use self::{
    data_series_encodings::DataSeriesEncodings, encoding::Encoding,
    preservation_map::PreservationMap, tag_encodings::TagEncodings,
};

/// A CRAM container compression header.
///
/// The compression header has three maps with information about how the data is compressed: a
/// preservation map, a data series encodings, and a tag encoding map.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompressionHeader {
    pub(crate) preservation_map: PreservationMap,
    pub(crate) data_series_encodings: DataSeriesEncodings,
    pub(crate) tag_encodings: TagEncodings,
}

impl CompressionHeader {
    pub(crate) fn new(
        preservation_map: PreservationMap,
        data_series_encodings: DataSeriesEncodings,
        tag_encodings: TagEncodings,
    ) -> Self {
        Self {
            preservation_map,
            data_series_encodings,
            tag_encodings,
        }
    }

    pub(crate) fn preservation_map(&self) -> &PreservationMap {
        &self.preservation_map
    }

    pub(crate) fn data_series_encodings(&self) -> &DataSeriesEncodings {
        &self.data_series_encodings
    }

    pub(crate) fn tag_encodings(&self) -> &TagEncodings {
        &self.tag_encodings
    }
}
