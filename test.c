#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <net/if.h>
#include <sys/ioctl.h>
#include <fcntl.h>
#include <linux/if_arp.h> // Add this header for ARPHRD_ETHER
#include <iostream>

int setMacAddress(const char *interface_name, const unsigned char *new_mac_address) {
    std::cout << "0";

    struct ifreq ifr;
    int sockfd;

    std::cout << "1";
    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd < 0) {
        perror("socket");
        return -1;
    }
    std::cout << "2";

    // Get the interface index
    strncpy(ifr.ifr_name, interface_name, IFNAMSIZ - 1);
    ifr.ifr_name[IFNAMSIZ - 1] = '\0';

    std::cout << "3";
    // Get the interface index
    if (ioctl(sockfd, SIOCGIFINDEX, &ifr) == -1) {
        perror("ioctl");
        close(sockfd);
        return -1;
    }

    std::cout << "4";
    // Set the interface down before changing the MAC address
    ifr.ifr_flags &= ~IFF_UP;
    if (ioctl(sockfd, SIOCSIFFLAGS, &ifr) == -1) {
        perror("ioctl");
        close(sockfd);
        return -1;
    }

    // Set the new MAC address
    memcpy(ifr.ifr_hwaddr.sa_data, new_mac_address, 6);
    std::cout << std::endl;
    for (auto& i : ifr.ifr_hwaddr.sa_data) {
        std::cout << (short)i << " ";
    }
    std::cout << std::endl;
    ifr.ifr_hwaddr.sa_family = 1; // ARPHRD_ETHER value if available, otherwise use 1

    if (ioctl(sockfd, SIOCSIFHWADDR, &ifr) == -1) {
        perror("ioctl");
        close(sockfd);
        return -1;
    }

    // Set the interface back up
    ifr.ifr_flags |= IFF_UP;
    if (ioctl(sockfd, SIOCSIFFLAGS, &ifr) == -1) {
        perror("ioctl");
        close(sockfd);
        return -1;
    }

    close(sockfd);
    return 0;
}

int main() {
    const char *interface_name = "enp7s0"; // Replace "eth0" with your desired interface name
    unsigned char new_mac_address[6] = {0x12, 0x98, 0x65, 0xa7, 0xa9, 0xce}; // Replace this with your desired MAC address

    std::cout << "-1";
    if (setMacAddress(interface_name, new_mac_address) == 0) {
        printf("MAC Address of %s changed to %02X:%02X:%02X:%02X:%02X:%02X\n",
               interface_name,
               new_mac_address[0], new_mac_address[1], new_mac_address[2],
               new_mac_address[3], new_mac_address[4], new_mac_address[5]);
    } else {
        printf("Failed to set MAC Address.\n");
    }

    return 0;
}
