use std::process::Command;
use std::str;
use std::collections::HashMap;
use colored::*;

fn signal_to_text(strength: i32) -> String {
    match strength {
        0..=20 => strength.to_string().truecolor(139, 0, 0).to_string(), // Dark red
        21..=50 => strength.to_string().truecolor(255, 165, 0).to_string(), // Orange
        51..=80 => strength.to_string().truecolor(255, 255, 0).to_string(), // Yellow
        81..=100 => strength.to_string().green().to_string(), // Green
        _ => strength.to_string(),
    }
}

fn list_wifi_networks() {
    let mut hash_map: HashMap<String, (String, i32)> = HashMap::new();

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("nmcli")
            .arg("-t")
            .arg("-f")
            .arg("SSID,BSSID,SIGNAL")
            .arg("dev")
            .arg("wifi")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let wifi_list = String::from_utf8_lossy(&output.stdout);
            for wifi in wifi_list.lines() {
                if let Some((mac_address_and_ssid, signal_strength)) = wifi.rsplit_once(":") {
                    if let Ok(strength) = signal_strength.parse::<i32>() {
                        let mut parts = mac_address_and_ssid.splitn(2, ":");
                        if let Some(name) = parts.next() {
                            if let Some(mac_address) = parts.next() {
                                if !name.trim().is_empty() && !mac_address.trim().is_empty() {
                                    hash_map.insert(name.to_string(), (
                                        mac_address.to_string(),
                                        strength,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            println!("Available Networks: ");
            for (name, (mac, strength)) in hash_map.iter() {
                println!(
                    "SSID: {}  BSSID: {}  Strength: {}%",
                    name.green(),
                    mac.replace("\\", "").yellow(),
                    signal_to_text(*strength)
                );
            }
        } else {
            eprintln!("Error: {}", str::from_utf8(&output.stderr).unwrap());
        }
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("netsh")
            .arg("wlan")
            .arg("show")
            .arg("network")
            .arg("mode=bssid")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let wifi_list = String::from_utf8_lossy(&output.stdout);
            let mut hash_map: HashMap<String, (String, i32)> = HashMap::new();
            for line in wifi_list.lines() {
                if line.contains("SSID") {
                    if let Some(ssid) = line.split(":").nth(1) {
                        let ssid = ssid.trim().to_string();
                        if let Some(bssid_line) = wifi_list.lines().find(|&l| l.contains("BSSID")) {
                            if let Some(bssid) = bssid_line.split(":").nth(1) {
                                let bssid = bssid.trim().to_string();
                                if
                                    let Some(signal_line) = wifi_list
                                        .lines()
                                        .find(|&l| l.contains("Signal"))
                                {
                                    if let Some(signal_strength) = signal_line.split(":").nth(1) {
                                        if let Ok(strength) = signal_strength.trim().parse::<i32>() {
                                            hash_map.insert(ssid, (bssid, strength));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            println!("Available Networks: ");
            for (name, (mac, strength)) in hash_map.iter() {
                println!(
                    "SSID: {}  BSSID: {}  Strength: {}",
                    name.green(),
                    mac.yellow(),
                    signal_to_text(*strength)
                );
            }
        } else {
            eprintln!("Error: {}", str::from_utf8(&output.stderr).unwrap());
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("airport").arg("-s").output().expect("Failed to execute command");

        if output.status.success() {
            let wifi_list = String::from_utf8_lossy(&output.stdout);
            let mut hash_map: HashMap<String, (String, i32)> = HashMap::new();

            for line in wifi_list.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let ssid = parts[0].to_string();
                    let bssid = parts[1].to_string();
                    if let Ok(strength) = parts[2].parse::<i32>() {
                        hash_map.insert(ssid, (bssid, strength));
                    }
                }
            }

            println!("Available Networks: ");
            for (name, (mac, strength)) in hash_map.iter() {
                println!(
                    "SSID: {}  BSSID: {}  Strength: {}",
                    name.green(),
                    mac.yellow(),
                    signal_to_text(*strength)
                );
            }
        } else {
            eprintln!("Error: {}", str::from_utf8(&output.stderr).unwrap());
        }
    }
}

fn main() {
    list_wifi_networks();
}
