//! # rederive — geometry as a derivation of the record
//!
//! `rederive` is the binding layer between an identity-bearing record
//! layer (IMBUUID) and the OpenGeometry geometry kernel. It stands
//! alone: applications depend on it; it belongs to none of them. The
//! ITPM app is its first consumer, not its owner.
//!
//! ## The one idea
//!
//! Geometry is a *function of the record*, never a copy of source
//! geometry. A record carries parameters; this crate hands those
//! parameters across a narrow seam to the kernel; the kernel derives
//! an exact solid, its measure, and its validity. Source geometry
//! (e.g. the geometry inside a Revit file) is used only as a *check*
//! the derivation is graded against — never as the source.
//!
//! ## The seam (boundary law)
//!
//! - Data crosses the seam: parameters down, measures up.
//! - Code never crosses: no record-layer internals live in this
//!   crate, and none of this crate's code enters the kernel tree.
//! - The kernel is a pinned git dependency (see `Cargo.toml`),
//!   wrapped, unmodified, never vendored.
//! - Records are named here by **opaque id only**. The record format
//!   carries IMBUUID addressing; how an address is formed is the
//!   record layer's own business and is deliberately not specified
//!   in this crate.
//!
//! ## Status
//!
//! Skeleton: the public interface is fixed as typed signatures with
//! contracts. Bodies land with the first measured derivation proof
//! (see `docs/OPEN-ITEMS.md` in the repository).

pub mod connectors;

/// An opaque record identifier.
///
/// The id is an IMBUUID address rendered as text by the record layer.
/// This crate treats it as opaque: it is carried into every
/// [`Derivation`] so a derived solid is always receipted against the
/// record it was derived from, but it is never parsed here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordId(pub String);

/// The parameter payload of a record, as handed across the seam.
///
/// Deliberately geometry-free: a type tag plus named scalar parameters
/// (dimensions, counts, layer thicknesses). Profiles and paths are
/// expressed as parameters of the type, not as imported geometry —
/// that is what makes the derivation a re-derivation (Route B) rather
/// than an import (Route A, rejected; see `docs/DECISIONS.md`).
#[derive(Debug, Clone, PartialEq)]
pub struct RecordParams {
    /// Type tag from the record's type catalog (e.g. a wall type, a
    /// slab type, an equipment type). The catalog itself lives with
    /// the record layer; this crate only carries the tag.
    pub type_tag: String,
    /// Named scalar parameters, in the record's own units contract
    /// (SI metres at the seam).
    pub scalars: Vec<(String, f64)>,
}

/// The kernel's validity verdict for a derived solid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validity {
    /// The solid is a valid B-rep.
    pub valid: bool,
    /// The solid is closed (watertight).
    pub closed: bool,
}

/// A handle to a derived solid held by the kernel side of the seam.
///
/// The solid itself never crosses back as data; callers hold this
/// reference and ask for measures, validity, or an exchange body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolidRef {
    /// Kernel-side body identifier for the derived solid.
    pub body: String,
}

/// The result of one derivation: a record, re-derived.
///
/// Every field is checkable: `volume` and `surface_area` are kernel
/// measures, `validity` is a kernel verdict, and `step` is the
/// exchange body a downstream writer can emit. A derivation is only
/// ever reported together with the [`RecordId`] it came from.
#[derive(Debug, Clone, PartialEq)]
pub struct Derivation {
    /// The record this geometry was derived from.
    pub record: RecordId,
    /// Handle to the derived solid.
    pub solid_ref: SolidRef,
    /// Kernel-measured volume, cubic metres.
    pub volume: f64,
    /// Kernel-measured surface area, square metres.
    pub surface_area: f64,
    /// Kernel validity verdict for the solid.
    pub validity: Validity,
    /// STEP (part21) exchange body of the derived solid.
    pub step: Vec<u8>,
}

/// A request to derive geometry from one record.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivationRequest {
    /// The record to derive from.
    pub record: RecordId,
    /// The record's parameters, as stored on the record.
    pub params: RecordParams,
}

/// The outcome of grading a [`Derivation`] against source geometry.
///
/// Source geometry is the check, never the source: this report is how
/// a derivation earns trust, by standing next to the measured source
/// it was re-derived from and showing the delta.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckReport {
    /// The record whose derivation was checked.
    pub record: RecordId,
    /// Kernel-derived volume, cubic metres.
    pub derived_volume: f64,
    /// Source-measured volume, cubic metres.
    pub source_volume: f64,
    /// Absolute delta between the two, cubic metres.
    pub delta: f64,
}

/// Ways a derivation can fail at the seam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeriveError {
    /// The record's type tag has no derivation in the type catalog.
    UnknownType(String),
    /// A required named parameter is absent from the record.
    MissingParam(String),
    /// The kernel refused the derivation (boolean failure, invalid
    /// profile, degenerate solid). Carries the kernel's message.
    Kernel(String),
}

impl std::fmt::Display for DeriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeriveError::UnknownType(t) => write!(f, "unknown record type: {t}"),
            DeriveError::MissingParam(p) => write!(f, "missing record parameter: {p}"),
            DeriveError::Kernel(m) => write!(f, "kernel derivation failed: {m}"),
        }
    }
}

impl std::error::Error for DeriveError {}

/// Derive geometry from a record.
///
/// This is the whole crate in one function: parameters down across
/// the seam, a solid, its measures, and its validity back up. The
/// same record parameters always derive the same solid — the kernel
/// is deterministic, so a derivation can be re-run and re-checked at
/// any time, by anyone holding the record.
pub fn derive(req: &DerivationRequest) -> Result<Derivation, DeriveError> {
    let _ = req;
    unimplemented!("skeleton: interface fixed, body lands with the first measured proof")
}

/// Grade a derivation against the source geometry it was re-derived
/// from. The source is the check, never the source.
pub fn check(derivation: &Derivation, source_volume: f64) -> CheckReport {
    CheckReport {
        record: derivation.record.clone(),
        derived_volume: derivation.volume,
        source_volume,
        delta: (derivation.volume - source_volume).abs(),
    }
}
