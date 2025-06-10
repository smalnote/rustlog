use std::fmt::Debug;

#[derive(Debug)]
pub struct Timestamp(pub prost_types::Timestamp);

impl Timestamp {
    const NANO: i64 = 1_000_000_000;
}

impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        let nanos = value.timestamp_nanos_opt().unwrap();
        let seconds = nanos / Self::NANO;
        let nanos = (nanos % Self::NANO) as i32;
        Self(prost_types::Timestamp { seconds, nanos })
    }
}

impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(value: Timestamp) -> Self {
        chrono::DateTime::from_timestamp(value.0.seconds, value.0.nanos as u32).unwrap()
    }
}
