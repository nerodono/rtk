use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Device {
    #[serde(rename_all = "camelCase")]
    Device {
        mac: String,
        vendor: String,
        ip: String,
        host_name: String,
        description: String,
        os: String,
        link_type: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "deviceList")]
pub struct Devices {
    #[serde(rename = "$value")]
    pub device_list: Vec<Device>,
}
