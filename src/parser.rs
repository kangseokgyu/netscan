use core::fmt;
use std::net::Ipv4Addr;

use macaddr::MacAddr6;
use regex::Regex;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

pub struct Device {
    ip: Ipv4Addr,
    mac: MacAddr6,
    vendor: String,
    os: String,
}

impl Device {
    pub fn parse(str: &str) -> Vec<Device> {
        let mut devices = Vec::new();

        let reg = Regex::new(
            r"Nmap scan report for (?<ip>.+)\nHost is up.*\nMAC Address: (?<mac>[0-9A-F:]{17}) \((?<vendor>.+)\)\n"
        ).unwrap();

        for (_, [ip, mac, vendor]) in reg.captures_iter(str).map(|c| c.extract()) {
            devices.push(Device {
                ip: ip.parse().unwrap(),
                mac: mac.parse().unwrap(),
                vendor: vendor.to_string(),
                os: "unknown".to_string(),
            });
        }

        devices
    }
}

impl fmt::Display for Device {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "IP: {} | MAC: {} | Vendor: {}, | OS: {}",
            self.ip, self.mac, self.vendor, self.os
        )
    }
}

impl Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Device", 4)?;
        state.serialize_field("ip", &self.ip.to_string())?;
        state.serialize_field("mac", &self.mac.to_string())?;
        state.serialize_field("vendor", &self.vendor)?;
        state.serialize_field("os", &self.os)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::Device;

    #[test]
    fn test1() {
        let log = "
Starting Nmap 7.60 ( https://nmap.org ) at 2024-04-26 04:44 UTC
Nmap scan report for 192.168.1.19
Host is up (-0.10s latency).
MAC Address: 02:00:00:00:00:01 (Unknown)
Nmap scan report for 192.168.1.1
Host is up.
Nmap done: 8192 IP addresses (2 hosts up) scanned in 80.29 seconds
";
        let devices = Device::parse(log);
        for d in devices.iter() {
            println!("{}", d);
        }

        println!("{}", serde_json::to_string_pretty(&devices).unwrap());
    }
}
