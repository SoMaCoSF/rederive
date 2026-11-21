//! # Connectors — the adoption mechanics
//!
//! A connector is the mapping layer between IMBUUID-addressed records
//! and one tool in the 3D / geospatial world (a Revit file, an
//! Onshape document, a Blender scene, a kit catalog). Every tool
//! that pipes through `rederive` speaks IMBUUID: it hands records in
//! by address and gets records back by address.
//!
//! ## The contract (one shape for all connectors)
//!
//! 1. **INGEST** — a source (file, document, scene) is read by the
//!    connector and lands as addressed records. Address assignment is
//!    the record layer's act; the connector carries the ids it was
//!    given and never invents identity semantics.
//! 2. **DERIVE** — geometry for those records is produced only
//!    through [`crate::derive`]: parameters down across the seam,
//!    solid / measure / validity up. A connector never imports
//!    foreign geometry as truth (Route B; source geometry is the
//!    check, never the source).
//! 3. **EMIT** — records are written back out in the source's format,
//!    each emitted object traceable to the record id it came from.
//!
//! **Receipts:** every transform a connector performs — ingest,
//! derivation, emit — is receipted against the record ids it
//! touched. A transform with no record ids is not a connector
//! transform; it is an untracked side effect, and it does not ship.
//!
//! ## Boundary
//!
//! Connector code in this crate is ours and shared (MPL-2.0).
//! IMBUUID internals stay out: connectors see opaque
//! [`crate::RecordId`]s and record parameters, nothing more.
//!
//! ## Register
//!
//! One stub module per named connector, each carrying its honest
//! state in its docs: [`revit`], [`onshape`], [`blender`],
//! [`anymachine`]. Interface only — no implementations live here yet,
//! and no state in a stub's docs may be read as stronger than it says.

pub mod anymachine;
pub mod blender;
pub mod onshape;
pub mod revit;

use crate::{Derivation, RecordId};

/// A source a connector can ingest, named in the source world's own
/// terms (a file path, a document id). Carried as text: the
/// connector resolves it; this crate does not open sources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRef(pub String);

/// A target a connector can emit to, in the source world's terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetRef(pub String);

/// The receipt of one connector transform: what was touched, and
/// what came out. Receipts are how a derivation earns trust — every
/// id named here can be re-derived and re-checked.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformReceipt {
    /// Record ids the transform touched.
    pub records: Vec<RecordId>,
    /// Derivations produced, where the transform derived geometry.
    pub derivations: Vec<Derivation>,
}

/// Ways a connector transform can fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectorError {
    /// The source could not be read at all.
    SourceUnreadable(String),
    /// The source read, but an element could not be carried into a
    /// record (named, never silently dropped).
    UnmappableElement(String),
    /// The target refused the emit.
    EmitRefused(String),
}

impl std::fmt::Display for ConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectorError::SourceUnreadable(m) => write!(f, "source unreadable: {m}"),
            ConnectorError::UnmappableElement(m) => write!(f, "unmappable element: {m}"),
            ConnectorError::EmitRefused(m) => write!(f, "emit refused: {m}"),
        }
    }
}

impl std::error::Error for ConnectorError {}

/// The connector contract, as a trait. One shape for all connectors:
/// ingest to records, derive through the kernel, emit from records,
/// receipt everything against the record ids touched.
pub trait Connector {
    /// The source world's name for this connector (e.g. `"revit"`).
    fn name(&self) -> &'static str;

    /// INGEST: read a source and land it as addressed records.
    /// Returns the ids of the records created or resolved, with the
    /// transform receipt.
    fn ingest(&self, source: &SourceRef) -> Result<TransformReceipt, ConnectorError>;

    /// EMIT: write records back out in the source's format. Every
    /// emitted object traces to a record id in the receipt.
    fn emit(
        &self,
        records: &[RecordId],
        target: &TargetRef,
    ) -> Result<TransformReceipt, ConnectorError>;
}
