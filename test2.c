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
#include <iostream>

int getMacAddress(const char *interface_name, unsigned char *mac_address) {
    struct ifreq ifr;
    int sockfd;

    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd < 0) {
        perror("socket");
        return -1;
    }

    strncpy(ifr.ifr_name, interface_name, IFNAMSIZ - 1);
    ifr.ifr_name[IFNAMSIZ - 1] = '\0';

    const char* ptr = ifr.ifr_name;
    while (*ptr != '\0') {
        std::cout << "Byte: " << static_cast<unsigned>(*ptr) << std::endl;
        ptr++;
    }

    if (ioctl(sockfd, SIOCGIFHWADDR, &ifr) == -1) {
        perror("ioctl");
        close(sockfd);
        return -1;
    }

    memcpy(mac_address, ifr.ifr_hwaddr.sa_data, 6);

    close(sockfd);
    return 0;
}

int main() {
    const char *interface_name = "enp7s0";
    unsigned char mac_address[6];

    if (getMacAddress(interface_name, mac_address) == 0) {
        std::cout << mac_address[1];
        printf("MAC Address of %s: %02X:%02X:%02X:%02X:%02X:%02X\n",
               interface_name,
               mac_address[0], mac_address[1], mac_address[2],
               mac_address[3], mac_address[4], mac_address[5]);
    } else {
        printf("Failed to get MAC Address.\n");
    }

    return 0;
}

