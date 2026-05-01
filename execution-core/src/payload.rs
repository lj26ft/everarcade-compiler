use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Payload {
    #[serde(rename = "const")]
    Const { value: String },

    #[serde(rename = "concat")]
    Concat { with: String },
}

impl Payload {
    pub fn validate(&self) {
        match self {
            Payload::Const { value } => {
                assert!(
                    !value.is_empty(),
                    "Const payload must not be empty"
                );
            }
            Payload::Concat { with } => {
                assert!(
                    !with.is_empty(),
                    "Concat payload must not be empty"
                );
            }
        }
    }
}
