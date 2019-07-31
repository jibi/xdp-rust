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

#[repr(C)]
pub struct TcpHdr {
    pub source: u16,
    pub dest: u16,
    pub seq: u32,
    pub ack_seq: u32,
    pub flags: u16,
    pub window: u16,
    pub check: u16,
    pub urg_ptr: u16,
}

pub struct Tcp<'a> {
    pub hdr: &'a mut TcpHdr,
    pub payload: *const u8,
    pub payload_end: *const u8,
}

impl Tcp<'_> {
    pub const fn port(port: u16) -> u16 {
        port.to_be()
    }
}
