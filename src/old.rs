use std::io::{self};
use std::ffi::CString;
use libc::{c_char, sockaddr, socket, AF_INET, SOCK_DGRAM, IFNAMSIZ, SIOCGIFHWADDR, ioctl};
use libc::ifreq;
use libc::__c_anonymous_ifr_ifru as IfReq;

fn get_mac_address(interface_name: &str) -> io::Result<[c_char; 6]> {
    let mut if_name: [c_char; IFNAMSIZ] = ['\0' as c_char; IFNAMSIZ];

    for i in 0..interface_name.as_bytes().len() {
        if_name[i] = interface_name.as_bytes()[i] as c_char;
    }

    let mut ifr: ifreq = ifreq {
        ifr_name: if_name,
        ifr_ifru: IfReq {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };

    println!("name:");
    for char in ifr.ifr_name.iter() {
        println!("{}", *char as u8 as char);
    }

    let sockfd = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };

    if sockfd == -1 {
        return Err(io::Error::last_os_error());
    }

    // Convert the interface name to a C-style string.
    let if_name_cstr = CString::new(interface_name).expect("CString::new failed");

    // Get the interface index.
    if unsafe { ioctl(sockfd, SIOCGIFHWADDR, &mut ifr) } == -1 {
        return Err(io::Error::last_os_error());
    }

    unsafe {
        libc::close(sockfd);
    }

    // Copy the MAC address to the output parameter.
    unsafe {
        dbg!(ifr.ifr_ifru);
    }
    let mac_address = unsafe{ &ifr.ifr_ifru.ifru_hwaddr.sa_data[..6] };
    Ok(mac_address.try_into().unwrap())
}

fn main() {
    const INTERFACE_NAME: &str = "enp7s0";

    match get_mac_address(INTERFACE_NAME) {
        Ok(addr) => {
            println!(
                "MAC Address of {}: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                INTERFACE_NAME,
                addr[0], addr[1], addr[2],
                addr[3], addr[4], addr[5]
            );
        }
        Err(err) => {
            eprintln!("Failed to get MAC Address: {}", err);
        }
    }
}
