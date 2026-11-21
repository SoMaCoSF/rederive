//! AnyMachine / Anykit connector — the kit-as-data line: catalogs of
//! typed parts in, addressed records out.
//!
//! **Honest state: CANDIDATE. REA not yet run.**
//!
//! This stub names a direction, not a result. The kit-as-data
//! judgment stands on other evidence (Floorkit's published pattern,
//! SOM-DOC-10902; the estate's own kit judgment from the IVF work):
//! author the kit once as data — type definitions carrying layered
//! property sheets — and derive geometry, drawings and quantities
//! from element records that reference the types. AnyMachine/Anykit
//! is the candidate source line for machine/kit catalogs in that
//! shape. No REA (acquire → exercise → measure → extract) has been
//! run against it, no license has been verified, and nothing is
//! absorbed. This module exists so the register is complete and the
//! owed work is visible.
//!
//! Shape under the contract, if the REA lands: INGEST kit catalog →
//! type records + element records · DERIVE via [`crate::derive`] ·
//! EMIT records → kit solids and quantities.

use super::{Connector, ConnectorError, SourceRef, TargetRef, TransformReceipt};
use crate::RecordId;

/// The AnyMachine/Anykit connector. Interface only; candidate.
pub struct AnyMachineConnector;

impl Connector for AnyMachineConnector {
    fn name(&self) -> &'static str {
        "anymachine"
    }

    fn ingest(&self, source: &SourceRef) -> Result<TransformReceipt, ConnectorError> {
        let _ = source;
        unimplemented!("interface only: candidate — REA not yet run")
    }

    fn emit(
        &self,
        records: &[RecordId],
        target: &TargetRef,
    ) -> Result<TransformReceipt, ConnectorError> {
        let _ = (records, target);
        unimplemented!("interface only: candidate — REA not yet run")
    }
}
