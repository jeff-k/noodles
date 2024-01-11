//! SAM record and fields.

pub mod data;
pub mod quality_scores;
pub mod reference_sequence_name;
pub mod sequence;
pub mod template_length;

pub use self::{
    quality_scores::QualityScores, reference_sequence_name::ReferenceSequenceName,
    sequence::Sequence, template_length::TemplateLength,
};
