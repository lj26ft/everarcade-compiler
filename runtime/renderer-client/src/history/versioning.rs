#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalArchiveFormatVersion {
    pub major: u16,
    pub minor: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalArchiveCompatibility {
    pub compatible: bool,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalArchiveVersionManifest {
    pub archive_id: String,
    pub format: HistoricalArchiveFormatVersion,
    pub minimum_reader_major: u16,
}

impl HistoricalArchiveVersionManifest {
    pub fn compatibility(
        &self,
        reader: &HistoricalArchiveFormatVersion,
    ) -> HistoricalArchiveCompatibility {
        if reader.major < self.minimum_reader_major || reader.major != self.format.major {
            return HistoricalArchiveCompatibility {
                compatible: false,
                reason: "incompatible_major_version".into(),
            };
        }
        HistoricalArchiveCompatibility {
            compatible: true,
            reason: "compatible".into(),
        }
    }
}
