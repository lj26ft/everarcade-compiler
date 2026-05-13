use super::artifact::IpfsPublicationIntent;
pub fn validate_intent(i:&IpfsPublicationIntent)->bool{!i.artifact_path.is_empty()}
