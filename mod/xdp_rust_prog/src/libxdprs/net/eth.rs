//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use crate::libxdprs::net::ip::*;
use crate::libxdprs::xdp::*;

#[repr(C)]
pub struct EthHdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub proto: u16,
}

pub struct Eth<'a> {
    pub hdr: &'a mut EthHdr,
    payload: *const u8,
    payload_end: *const u8,
}

impl Eth<'_> {
    pub const ETHERTYPE_IP: u16 = (0x0800 as u16).to_be();

    pub fn new<'a>(xdp: *const XdpBuff) -> Result<Eth<'a>, XdpError> {
        let data = unsafe { *xdp };

        if unsafe { data.data.offset(14) } > data.data_end {
            return Err(XdpError::InvalidEthHdr);
        }

        Ok(Eth {
            hdr: unsafe { &mut *(data.data as *mut EthHdr) },
            payload: unsafe { data.data.offset(14) },
            payload_end: data.data_end,
        })
    }

    pub fn ip(&self) -> Result<Ip, XdpError> {
        if unsafe { self.payload.offset(20) } > self.payload_end {
            return Err(XdpError::InvalidIpHdr);
        }

        Ok(Ip {
            hdr: unsafe { &mut *(self.payload as *mut IpHdr) },
            payload: unsafe { self.payload.offset(20) },
            payload_end: self.payload_end,
        })
    }
}
