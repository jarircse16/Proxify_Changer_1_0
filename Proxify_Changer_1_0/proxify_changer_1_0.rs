extern crate winreg;

use std::error::Error;
use winreg::enums::*;
use winreg::RegKey;
use std::collections::HashMap;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        println!("Options:");
        println!("1. Add Proxy");
        println!("2. Remove Proxy");
        println!("3. Exit");
        println!("Enter your choice: ");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => add_proxy()?,
            2 => remove_proxy()?,
            3 => break,
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),
        }
    }

    Ok(())
}

fn add_proxy() -> Result<(), Box<dyn Error>> {
    println!("Enter the SOCKS4 proxy server: ");
    let mut proxy_server = String::new();
    io::stdin().read_line(&mut proxy_server)?;
    let proxy_server = proxy_server.trim();

    println!("Enter the proxy port: ");
    let mut proxy_port = String::new();
    io::stdin().read_line(&mut proxy_port)?;
    let proxy_port = proxy_port.trim();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let internet_settings = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
        KEY_WRITE,
    )?;
    
    let previous_settings = get_proxy_settings(&internet_settings)?;
    
    set_proxy(&internet_settings, proxy_server, proxy_port)?;

    println!("SOCKS4 proxy set to {}:{}", proxy_server, proxy_port);

    Ok(())
}

fn remove_proxy() -> Result<(), Box<dyn Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let internet_settings = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
        KEY_WRITE,
    )?;

    remove_proxy_settings(&internet_settings)?;

    println!("Proxy removed");

    Ok(())
}

fn set_proxy(
    internet_settings: &RegKey,
    proxy_server: &str,
    proxy_port: &str,
) -> Result<(), Box<dyn Error>> {
    internet_settings.set_value("ProxyEnable", &1u32)?;
    internet_settings.set_value("ProxyServer", &format!("{}:{}", proxy_server, proxy_port))?;
    internet_settings.set_value("ProxyOverride", &"<local>")?;

    Ok(())
}

fn get_proxy_settings(internet_settings: &RegKey) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut previous_settings = HashMap::new();

    if let Ok(proxy_enable) = internet_settings.get_value::<u32, _>("ProxyEnable") {
        if proxy_enable == 1 {
            if let Ok(proxy_server) = internet_settings.get_value::<String, _>("ProxyServer") {
                previous_settings.insert("ProxyServer".to_string(), proxy_server);
            }

            if let Ok(proxy_override) = internet_settings.get_value::<String, _>("ProxyOverride") {
                previous_settings.insert("ProxyOverride".to_string(), proxy_override);
            }
        }
    }

    Ok(previous_settings)
}

fn remove_proxy_settings(internet_settings: &RegKey) -> Result<(), Box<dyn Error>> {
    internet_settings.set_value("ProxyEnable", &0u32)?;
    internet_settings.set_value("ProxyServer", &"")?;
    internet_settings.set_value("ProxyOverride", &"")?;

    Ok(())
}
