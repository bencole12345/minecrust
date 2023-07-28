/// An MTU value that we assume will never be fragmented. This is a slightly conservative estimate
/// for modern systems.
const ASSUMED_MTU: usize = 1200;

struct DataPacket {
    sequence_num: u32,
    data: [u8]
}

struct AckPacket {
    sequence_num: u32
}

enum Packet {
    Data(DataPacket),
    Ack(AckPacket)
}

struct ManagedUDP {

}

impl ManagedUDP {
    /// Send a packet quickly.
    ///
    /// Each packet will be resent only if it has not been ACKed on the next tick.
    ///
    /// If a `class` is specified then future
    fn send(data: &[u8]) {
        todo!()
    }

    fn send_overridable(data: &[u8], class: u64) {
        todo!()
    }

    fn send_bulk_reliable(data: &[u8]) {
        todo!()
    }
}
