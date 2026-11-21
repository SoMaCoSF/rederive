//! Onshape connector — Onshape documents in, addressed records out.
//!
//! **Honest state: connector BUILT and OFFLINE-VERIFIED; live use
//! BLOCKED on the founder's account tier.**
//!
//! The connector exists (SOM-DOC-10886): a read-only slurp that
//! lists documents, elements and parts, reads per-part metadata and
//! assembly BOMs, and emits rows in the encoder's ingest shape, with
//! source identity carried as
//! `onshape:<documentId>/<wvm><id>/e/<elementId>/p/<partId>` in the
//! keymap. Offline tests: 16/16 PASS.
//!
//! Live use is blocked, and the block is named, not vague: both of
//! the founder's Onshape accounts sit on the Pro Discovery tier,
//! which refuses API key creation **and** refuses OAuth app
//! registration. No credential path exists on that tier. Unblocking
//! is a tier change (a founder decision) or the answer to the open
//! question of how Airsup authenticates on the same tier.
//!
//! Shape under the contract: INGEST document → addressed records
//! (parts as records, assembly structure as membership) · DERIVE via
//! [`crate::derive`] · EMIT records → per-part derivation receipts
//! and exchange bodies.

use super::{Connector, ConnectorError, SourceRef, TargetRef, TransformReceipt};
use crate::RecordId;

/// The Onshape connector. Interface only.
pub struct OnshapeConnector;

impl Connector for OnshapeConnector {
    fn name(&self) -> &'static str {
        "onshape"
    }

    fn ingest(&self, source: &SourceRef) -> Result<TransformReceipt, ConnectorError> {
        let _ = source;
        unimplemented!("interface only: connector built + offline-verified in its own line (SOM-DOC-10886)")
    }

    fn emit(
        &self,
        records: &[RecordId],
        target: &TargetRef,
    ) -> Result<TransformReceipt, ConnectorError> {
        let _ = (records, target);
        unimplemented!("interface only")
    }
}
