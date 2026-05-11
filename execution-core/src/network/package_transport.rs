#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportPacket {
    pub package_id: String,
    pub chunks: Vec<Vec<u8>>,
    pub digest: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportReceipt {
    pub package_id: String,
    pub chunk_count: usize,
    pub digest: u64,
}

#[derive(Debug, Default)]
pub struct TransportService;

impl TransportService {
    pub fn package(package_id: &str, bytes: &[u8], chunk_size: usize) -> TransportPacket {
        let size = chunk_size.max(1);
        let chunks = bytes.chunks(size).map(|c| c.to_vec()).collect::<Vec<_>>();
        TransportPacket {
            package_id: package_id.to_string(),
            digest: checksum(bytes),
            chunks,
        }
    }

    pub fn reconstruct(packet: &TransportPacket) -> Option<(Vec<u8>, TransportReceipt)> {
        let bytes = packet.chunks.iter().flat_map(|c| c.iter().copied()).collect::<Vec<_>>();
        let digest = checksum(&bytes);
        if digest != packet.digest {
            return None;
        }
        Some((
            bytes,
            TransportReceipt {
                package_id: packet.package_id.clone(),
                chunk_count: packet.chunks.len(),
                digest,
            },
        ))
    }
}

fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0u64, |acc, b| acc.wrapping_mul(16777619).wrapping_add(*b as u64))
}
