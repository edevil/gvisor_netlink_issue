use neli::consts::nl::{NlmF, NlmFFlags};
use neli::consts::rtnl::RtAddrFamily::Inet;
use neli::consts::rtnl::{IfaFFlags, Rtm};
use neli::consts::socket::NlFamily;
use neli::nl::{NlPayload, Nlmsghdr};
use neli::rtnl::Ifaddrmsg;
use neli::socket::NlSocketHandle;
use neli::types::RtBuffer;

fn main() {
    println!("Starting");
    log::set_max_level(log::Level::max().to_level_filter()); // somehow this global variable change triggers the behaviour
    println!("Crashing....");
    local_ip_impl();
    println!("Or not!");
}

fn local_ip_impl() {
    let mut netlink_socket = NlSocketHandle::connect(NlFamily::Route, None, &[]).unwrap();

    let ifaddrmsg = Ifaddrmsg {
        ifa_family: Inet,
        ifa_prefixlen: 0,
        ifa_flags: IfaFFlags::empty(),
        ifa_scope: 0,
        ifa_index: 0,
        rtattrs: RtBuffer::new(),
    };
    let netlink_message = Nlmsghdr::new(
        None,
        Rtm::Getaddr,
        NlmFFlags::new(&[NlmF::Request, NlmF::Root]),
        None,
        None,
        NlPayload::Payload(ifaddrmsg),
    );

    netlink_socket.send(netlink_message).unwrap();

    let response = netlink_socket.iter(false).next();
    let _header: Result<Nlmsghdr<Rtm, Ifaddrmsg>, _> = response.unwrap();
}
