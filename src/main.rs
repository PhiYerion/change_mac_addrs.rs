use std::io;
use std::ffi::CString;
use std::mem::MaybeUninit;
use libc::{c_char, c_uchar, c_int, sockaddr, socket, AF_INET, SOCK_DGRAM, IFNAMSIZ, IFF_UP, SIOCGIFINDEX, SIOCSIFFLAGS, SIOCSIFHWADDR, ifreq, ioctl, ARPHRD_ETHER};

unsafe fn set_mac_address(interface_name: &str, new_mac_address: &[c_char; 6]) -> io::Result<()> {
    let mut if_name: [c_char; IFNAMSIZ] = ['\0' as c_char; IFNAMSIZ];

    for i in 0..interface_name.as_bytes().len() {
        if_name[i] = interface_name.as_bytes()[i] as c_char;
    }

    let mut ifr: ifreq = ifreq {
        ifr_name: if_name,
        ifr_ifru: libc::__c_anonymous_ifr_ifru {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };

    let sockfd = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };

    if sockfd == -1 {
        return Err(io::Error::last_os_error());
    }

    // Get the interface index.
    let interface_index = unsafe { ioctl(sockfd, SIOCGIFINDEX, &mut ifr) };
    dbg!(interface_index);
    if interface_index == -1 {
        return Err(io::Error::last_os_error());
    }

    // Set the interface down before changing the MAC address.
    ifr.ifr_ifru.ifru_flags &= !(IFF_UP as i16);
    if unsafe { ioctl(sockfd, SIOCSIFFLAGS, &ifr) } == -1 {
        return Err(io::Error::last_os_error());
    }

    // Set the new MAC address.
    let mut old = ifr.ifr_ifru.ifru_hwaddr.sa_data;
    old[0] = new_mac_address[0];
    old[1] = new_mac_address[1];
    old[2] = new_mac_address[2];
    old[3] = new_mac_address[3];
    old[4] = new_mac_address[4];
    old[5] = new_mac_address[5];
    ifr.ifr_ifru.ifru_hwaddr.sa_data = old;

    ifr.ifr_ifru.ifru_hwaddr.sa_family = ARPHRD_ETHER;

    if unsafe { ioctl(sockfd, SIOCSIFHWADDR, &ifr) } == -1 {
        return Err(io::Error::last_os_error());
    }

    // Set the interface back up.
    ifr.ifr_ifru.ifru_flags |= IFF_UP as i16;
    if unsafe { ioctl(sockfd, SIOCSIFFLAGS, &ifr) } == -1 {
        return Err(io::Error::last_os_error());
    }

    unsafe {
        libc::close(sockfd);
    }

    Ok(())
}

fn main() {
    const INTERFACE_NAME: &str = "enp7s0"; // Replace "eth0" with your desired interface name
    let new_mac_address: [c_char; 6] = [0x12u8 as c_char, 0x34u8 as c_char, 0x56u8 as c_char, 0x78u8 as c_char, 0x9au8 as c_char, 0xbcu8 as c_char]; // Replace this with your desired MAC address

    unsafe {
        match set_mac_address(INTERFACE_NAME, &new_mac_address) {
            Ok(_) => {
                println!(
                    "MAC Address of {} changed to {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    INTERFACE_NAME,
                    new_mac_address[0], new_mac_address[1], new_mac_address[2],
                    new_mac_address[3], new_mac_address[4], new_mac_address[5]
                );
            }
            Err(err) => {
                eprintln!("Failed to set MAC Address: {}", err);
            }
        }
    }
}
