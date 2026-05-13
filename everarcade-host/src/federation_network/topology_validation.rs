use super::topology::FederationTopology;
pub fn topology_valid(topology: &FederationTopology) -> bool { topology.member_count > 0 }
