use axum::{http::StatusCode, response::IntoResponse};

use crate::device::Device;

/// Returns a list of connected devices.
pub async fn devices() -> impl IntoResponse {
    let devices = [
        Device {
            uuid: "b0e42fe7-31a5-4894-a441-007e5256afea".to_string(),
            mac: "5F-33-CC-1F-43-82".to_string(),
            firmware: "2.1.6".to_string(),
        },
        Device {
            uuid: "0c3242f5-ae1f-4e0c-a31b-5ec93825b3e7".to_string(),
            mac: "EF-2B-C4-F5-D6-34".to_string(),
            firmware: "2.1.5".to_string(),
        },
        Device {
            uuid: "b16d0b53-14f1-4c11-8e29-b9fcef167c26".to_string(),
            mac: "62-46-13-B7-B3-A1".to_string(),
            firmware: "3.0.0".to_string(),
        },
        Device {
            uuid: "51bb1937-e005-4327-a3bd-9f32dcf00db8".to_string(),
            mac: "96-A8-DE-5B-77-14".to_string(),
            firmware: "1.0.1".to_string(),
        },
        Device {
            uuid: "e0a1d085-dce5-48db-a794-35640113fa67".to_string(),
            mac: "7E-3B-62-A6-09-12".to_string(),
            firmware: "3.5.6".to_string(),
        },
    ];

    let devices = serde_json::to_string(&devices).unwrap();
    let devices: Vec<Device> = serde_json::from_str(&devices).unwrap();

    (StatusCode::OK, axum::Json(devices))
}
