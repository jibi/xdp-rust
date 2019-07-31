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

#![crate_type = "staticlib"]
#![feature(lang_items)]
#![no_std]

mod libxdprs;
use libxdprs::{XdpAction::*, *};

#[inline]
fn do_match(xdp_buff: *const XdpBuff) -> Result<u32, XdpError> {
    let eth = Eth::new(xdp_buff)?;
    if eth.hdr.proto != Eth::ETHERTYPE_IP {
        return XdpPass.into();
    }

    let ip = eth.ip()?;
    if ip.hdr.daddr != Ip::addr(192, 168, 122, 193) {
        return XdpPass.into();
    }

    let tcp = ip.tcp()?;
    if tcp.hdr.dest != Tcp::port(1234) {
        return XdpPass.into();
    }

    XdpDrop.into()
}

#[no_mangle]
pub fn xdp_rust_prog(xdp: *const XdpBuff) -> u32 {
    match do_match(xdp) {
        Ok(ret) => ret,
        Err(_) => XdpAborted.into(),
    }
}
