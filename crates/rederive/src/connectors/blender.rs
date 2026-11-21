//! Blender connector — the render / emit side: records out to
//! scenes, renders, and turntables.
//!
//! **Honest state: render/emit side only; real execution
//! UNVERIFIED.**
//!
//! Blender's role in this line is presentation of derivations, not
//! derivation itself: a record's kernel solid, rendered — the same
//! posture as the sprite-stack turntables on the dossier surfaces.
//! The local Blender-driving code that exists in the estate (the
//! hexlab line) passes its local test suite, but real Blender
//! execution has never been acceptance-run on the founder's
//! hardware; on this VM it is graded UNVERIFIED, and this module
//! claims no more.
//!
//! Shape under the contract: INGEST is not Blender's direction (it
//! originates no records) · DERIVE via [`crate::derive`] upstream of
//! the render · EMIT records → scene / render / turntable, each
//! frame receipted to the record ids it depicts.

use super::{Connector, ConnectorError, SourceRef, TargetRef, TransformReceipt};
use crate::RecordId;

/// The Blender connector. Interface only.
pub struct BlenderConnector;

impl Connector for BlenderConnector {
    fn name(&self) -> &'static str {
        "blender"
    }

    fn ingest(&self, source: &SourceRef) -> Result<TransformReceipt, ConnectorError> {
        let _ = source;
        unimplemented!("interface only: Blender originates no records in this line")
    }

    fn emit(
        &self,
        records: &[RecordId],
        target: &TargetRef,
    ) -> Result<TransformReceipt, ConnectorError> {
        let _ = (records, target);
        unimplemented!("interface only: real Blender execution UNVERIFIED")
    }
}
