<!--
Ghost Catalog: SOM-DOC-10904-v1.0.0
-->

# CONNECTORS — the adoption mechanics

`rederive` also carries the connector types: the mapping layer
between IMBUUID-addressed records and the 3D / geospatial tool
world. Connectors are how adoption happens — every tool that pipes
through rederive speaks IMBUUID: records in by address, records out
by address, geometry only ever as a derivation of the record.

## The contract (one shape for all)

```mermaid
flowchart LR
    SRC["Source world<br/>file · document · scene · catalog"]
    ING["INGEST<br/>source → addressed records"]
    DER["DERIVE<br/>via the kernel, at the seam"]
    EMI["EMIT<br/>records → source format"]
    SRC --> ING --> DER --> EMI --> SRC
    ING -. "receipt: record ids touched" .-> LED[("Transform receipts")]
    DER -. "receipt: record ids touched" .-> LED
    EMI -. "receipt: record ids touched" .-> LED
```

1. **INGEST** — a source is read and lands as addressed records.
   Address assignment is the record layer's act; a connector carries
   the ids it was given and never invents identity semantics.
2. **DERIVE** — geometry for those records comes only from the
   kernel derivation (`derive` in the crate). Parameters down,
   solid / measure / validity up. Source geometry is the check,
   never the source (Route B, DECISIONS (f)).
3. **EMIT** — records are written back in the source's format, each
   emitted object traceable to the record id it came from.

**Receipts:** every transform — ingest, derivation, emit — is
receipted against the record ids it touched. A transform with no
record ids is an untracked side effect, and it does not ship.

**Boundary:** connector code here is ours and shared (MPL-2.0).
IMBUUID internals stay out — connectors see opaque record ids and
record parameters, nothing more. The contract is a trait in the
crate (`connectors::Connector`); each register entry below has a
stub module carrying the contract as documentation.

## The register (honest states, no upgrades)

| Connector | Direction | State |
|---|---|---|
| **revit** | `.rvt` → records; records → IFC exchange bodies | Reader **PROVEN** by execution; pipe probe **IN FLIGHT** |
| **onshape** | Onshape documents → records | Connector **BUILT + OFFLINE-VERIFIED** (SOM-DOC-10886); live use **BLOCKED** on account tier |
| **blender** | Records → scenes / renders / turntables | Render/emit side; real execution **UNVERIFIED** |
| **anymachine / anykit** | Kit catalogs → type + element records | **CANDIDATE** — REA not yet run |

### revit

The reader half is proven by execution: the standalone reader
(rvt-rs, absorbed pinned in its own line) plus the encoder decoded
the Autodesk samples end to end — 542 elements with 100% of
parameters decoded and bound on the structural sample; on the MEP
samples, 238/238 and 240/240 systems recovered with 0 missed /
0 phantom under two-direction verification. No Revit process, no
bridge. The derivation half — one real element re-derived in-kernel
from its record, graded against the source — is the pipe probe,
in flight as this register is written. The Revit pipe is therefore
claimed as: proven reader, unproven derivation, probe running.

### onshape

The connector is built and offline-verified (SOM-DOC-10886):
read-only slurp of documents, elements, parts, metadata and BOMs,
emitting encoder-shaped rows with source identity carried in the
keymap; offline tests 16/16 — **re-run and re-verified 2026-11-21
on the current tree: 16/16 PASS this run**. The identity mapping
(`onshape:<documentId>/<wvm kind><wvm id>/e/<elementId>/p/<partId>`)
and the slurp row shape still match the encoder contract as
documented. One adapter gap, stated plainly: the encoder's current
keymap rows carry source-specific id fields (ElementId +
document GUID on the RVT lane) rather than a unified `foreign_key`
column, and the encoder does not yet ingest slurp JSONL — the
contract matches, the ingest adapter is not built. Live use is
blocked, and the block is named: both founder accounts are on
Onshape's Pro Discovery tier, which refuses API key creation
**and** OAuth app registration — no credential path exists on
that tier. Unblocking is a tier change (founder decision) or the
answer to the open question of how Airsup authenticates on the
same tier. No live claim is made.

### blender

Blender's role is the render/emit side: presentation of derivations
(scenes, renders, dossier turntables), never derivation itself.
The estate's Blender-driving code passes its local suite, but real
Blender execution has never been acceptance-run on the founder's
hardware — graded **UNVERIFIED**, and the register claims no more.
Freshness check 2026-11-21: no Blender binary is reachable on the
Witness VM at all — not on PATH, not in system dirs, not vendored
anywhere in the workspace trees (imbue/vendor holds ffmpeg, VLC
and WebView2 only; the one "blender" path in the tree is a
harvested documentation file). A headless emit proof therefore
cannot run on this VM; it runs on OMEN-01, where Blender lives
and which is the acceptance authority for this class of claim.
This also gates the lane below: AnyMachine/AnyKit sits **on**
Blender — Blender is the kit system's host — so the Blender host
proof (binary + version + `bpy` Python API in a headless run) is
the host proof for the AnyMachine connector, and it is owed on
OMEN-01, not here.

### anymachine / anykit

A candidate, recorded so the owed work is visible. The kit-as-data
judgment stands on other evidence (Floorkit's published pattern;
the estate's IVF kit judgment): author the kit once as data, derive
the rest. For this source line, no REA has been run, no license
verified, nothing absorbed. State: **CANDIDATE**. Host dependency
(founder, 2026-11-21): AnyMachine/AnyKit sits on Blender — when
the REA runs, its first question is the Blender host proof owed
in the blender entry above.
