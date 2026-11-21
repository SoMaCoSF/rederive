//! Revit connector — `.rvt` in, addressed records out; records back
//! to checked geometry.
//!
//! **Honest state: reader PROVEN by execution; pipe probe IN
//! FLIGHT.**
//!
//! The reader half is proven: the standalone reader (rvt-rs, absorbed
//! as a pinned dependency in its own line) plus the encoder decoded
//! the Autodesk sample projects end to end — elements, parameters
//! (100% decoded and bound on the structural sample), systems and
//! membership (238/238 and 240/240 systems recovered on the MEP
//! samples, 0 missed / 0 phantom, two-direction verified). No Revit
//! process, no bridge: the file is read directly.
//!
//! The derivation half — one real encoder element re-derived
//! in-kernel from its record's parameters, kernel volume graded
//! against the source's — is the pipe probe, running as this stub is
//! written. Until it reports, the Revit *pipe* is a proven reader
//! plus an unproven derivation, and this module claims exactly that.
//!
//! Shape under the contract: INGEST `.rvt` → addressed records
//! (commissionable / permittable things; architectural context rides
//! as ghost references, not records) · DERIVE via [`crate::derive`]
//! · EMIT records → IFC exchange bodies for downstream writers.

use super::{Connector, ConnectorError, SourceRef, TargetRef, TransformReceipt};
use crate::RecordId;

/// The Revit connector. Interface only.
pub struct RevitConnector;

impl Connector for RevitConnector {
    fn name(&self) -> &'static str {
        "revit"
    }

    fn ingest(&self, source: &SourceRef) -> Result<TransformReceipt, ConnectorError> {
        let _ = source;
        unimplemented!("interface only: reader proven in its own line, binding lands with the pipe proof")
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
