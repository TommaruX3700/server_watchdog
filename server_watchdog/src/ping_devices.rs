use pnet::packet::icmp::{echo_request, IcmpTypes};
use pnet::packet::{icmp, Packet, ip::IpNextHeaderProtocols};
use pnet::transport::{transport_channel, TransportChannelType::Layer3};
use std::fs;
use std::net::{IpAddr, ToSocketAddrs};
use std::io::{self, BufRead};

pub struct Pinger;

impl Pinger {
    // Function to ping an IP address
    pub fn ping(target: IpAddr) -> Result<(), String> {
        let protocol = IpNextHeaderProtocols::Icmp;
        let (mut tx, mut rx) = transport_channel(1024, Layer3(IpNextHeaderProtocols::Icmp)).map_err(|e| e.to_string())?;

        let mut packet = [0u8; 64];
        let mut icmp_packet = echo_request::MutableEchoRequestPacket::new(&mut packet).unwrap();
        icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
        icmp_packet.set_sequence_number(1);
        icmp_packet.set_identifier(42);
        let icmp_packet_immutable = echo_request::EchoRequestPacket::new(icmp_packet.packet()).unwrap();
        let icmp_packet_ref = icmp::IcmpPacket::new(icmp_packet_immutable.packet()).unwrap();
        let checksum = icmp::checksum(&icmp_packet_ref);
        icmp_packet.set_checksum(checksum);

        tx.send_to(icmp_packet, target)
            .map_err(|e| format!("Failed to send packet: {}", e))?;

        let mut packet_iter = pnet::transport::icmp_packet_iter(&mut rx);
        match packet_iter.next() {
            Ok(packet) => {
                println!("Received packet: {:?}", packet);
                Ok(())
            }
            Err(e) => Err(format!("Error receiving packet: {}", e)),
        }
    }

    // Function to read the file and return a list of hostnames
    pub fn read_hosts_from_file(filename: &str) -> Result<Vec<String>, io::Error> {
        let file = fs::File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut hosts = Vec::new();
        
        for line in reader.lines() {
            if let Ok(line) = line {
                let trimmed_line = line.trim();
                if !trimmed_line.is_empty() {
                    hosts.push(trimmed_line.to_string());
                }
            }
        }
        
        Ok(hosts)
    }

    // Function to resolve hostnames to IP addresses
    pub fn resolve_hostnames(hostnames: Vec<String>) -> Result<Vec<IpAddr>, String> {
        let mut ips = Vec::new();
        for hostname in hostnames {
            match hostname.to_socket_addrs() {
                Ok(mut addrs) => {
                    if let Some(ip) = addrs.next() {
                        ips.push(ip.ip());
                    } else {
                        return Err(format!("Could not resolve {}", hostname));
                    }
                }
                Err(e) => return Err(format!("Error resolving {}: {}", hostname, e)),
            }
        }
        Ok(ips)
    }
}
