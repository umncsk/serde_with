//! Internal Helper types

use crate::{formats::Strictness, DurationSeconds, SerializeAs};
use serde::{Serialize, Serializer};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub(crate) fn is_positive(&self) -> bool {
        *self == Sign::Positive
    }

    pub(crate) fn is_negative(&self) -> bool {
        *self == Sign::Negative
    }
}

pub(crate) struct DurationSigned {
    pub(crate) sign: Sign,
    pub(crate) duration: std::time::Duration,
}

impl<STRICTNESS> SerializeAs<DurationSigned> for DurationSeconds<i64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut secs = source.duration.as_secs() as i64;
        if source.sign.is_negative() {
            secs = -secs;
        }

        // Properly round the value
        if source.duration.subsec_millis() >= 500 {
            if source.sign.is_positive() {
                secs += 1;
            } else {
                secs -= 1;
            }
        }
        secs.serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<DurationSigned> for DurationSeconds<f64, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut secs = source.duration.as_secs() as f64;
        if source.sign.is_negative() {
            secs = -secs;
        }

        // Properly round the value
        if source.duration.subsec_millis() >= 500 {
            if source.sign.is_positive() {
                secs += 1.;
            } else {
                secs -= 1.;
            }
        }
        secs.serialize(serializer)
    }
}

impl<STRICTNESS> SerializeAs<DurationSigned> for DurationSeconds<String, STRICTNESS>
where
    STRICTNESS: Strictness,
{
    fn serialize_as<S>(source: &DurationSigned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut secs = source.duration.as_secs() as i64;
        if source.sign.is_negative() {
            secs = -secs;
        }

        // Properly round the value
        if source.duration.subsec_millis() >= 500 {
            if source.sign.is_positive() {
                secs += 1;
            } else {
                secs -= 1;
            }
        }
        secs.to_string().serialize(serializer)
    }
}
