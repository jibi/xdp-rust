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

use crate::libxdprs::net::tcp::*;
use crate::libxdprs::xdp::*;

#[repr(C)]
pub struct IpHdr {
    pub ver_ihl: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

pub struct Ip<'a> {
    pub hdr: &'a mut IpHdr,
    pub payload: *const u8,
    pub payload_end: *const u8,
}

impl Ip<'_> {
    pub const fn addr(x1: u8, x2: u8, x3: u8, x4: u8) -> u32 {
        let addr = (x1 as u32) << 24 | (x2 as u32) << 16 | (x3 as u32) << 8 | (x4 as u32);
        addr.to_be()
    }

    pub fn tcp(&self) -> Result<Tcp, XdpError> {
        let ihl = ((self.hdr.tos & 0x0f) * 4) as isize;

        if unsafe { self.payload.offset(ihl + 20) } > self.payload_end {
            return Err(XdpError::InvalidTcpHdr);
        }

        Ok(Tcp {
            hdr: unsafe { &mut *(self.payload.offset(ihl) as *mut TcpHdr) },
            payload: unsafe { self.payload.offset(ihl + 20) },
            payload_end: self.payload_end,
        })
    }
}
