# Plan: Lazy decoder (`TaskLazy`)

Discussion lock (2026-09-06). DESIGN.md §8 still describes an older per-field
rescan / `FieldCache` machine. This note is the implementation lock until that
section is rewritten. Do not start from DESIGN §8's cache state machine.

Hand-written sample first, same as eager. Plugin last.

## Why this, not DESIGN §8 as written

Last-wins makes **independent per-field scans** a weak design: a singular getter
must walk the whole parent stream, so N getters become N full scans.

The useful split is not “scan less of the parent”. It is:

- **One resumable wire scan** (can run as input arrives).
- **Materialise only expensive LEN** (string / bytes / message / map).
- **Numericals are applied when a complete record is in hand** — storing an
  offset is often larger and slower than storing the value.

Repeated numericals (including packed, for v1) follow the same rule: the scan
already visits every occurrence, so `push` into the existing `Vec` is cheaper
than a location list. Packed-as-slice is a later optimisation if huge unread
packed fields show up in numbers.

## Agreed product shape

### Public types

- Eager `Task<A>` stays the current catalog struct. Do **not** put a runtime
  eager/lazy enum inside it.
- Lazy is a **separate monomorphisation** with **different field types** where
  the slot must differ. Sizes are independent; eager does not pay for lazy
  storage.
- External names stay two aliases so eager signatures do not grow a layout
  parameter:

  ```rust
  pub type Task<A = Global> = TaskImpl<A, Eager>;
  pub type TaskLazy<A = Global> = TaskImpl<A, Lazy>;
  ```

  Do **not** introduce `TaskImpl` until both handwritten types exist and
  duplication hurts (step 9). Until then the sample types are `Task` and
  `TaskLazy`.

- Shared generic read surface: `TaskMessageFallible` (`Error = Infallible` on
  eager, `DecodeError` on lazy). Eager inherent `_mut` stays off the trait.
- Lazy is **read-oriented**. Mutation is `into_eager(self) -> Result<Task<A>,
  DecodeError>` (and then the existing mut API). No lazy `_mut` in v1.

### Parse model (hybrid B + incremental scan)

`merge_from` of two **complete** messages is protobuf merge (last-wins /
repeated-append). Feeding 8 KiB TCP/disk chunks of **one** message is
concatenation, not merge. Keep the APIs distinct internally:

- Scanner / `push(chunk)` — advance as far as complete records allow; keep
  leftover.
- `finish()` — leftover must be empty or `DecodeError` (truncated).
- `merge_from` — one complete message; may be `push` + `finish` on that buffer.

**Public getters are only valid after the input is finished.** Incremental parse
overlaps I/O with wire work. It does **not** expose last-wins fields before
EOF. Early `title()` would be a provisional value and cannot back out of
`&str`.

I/O into a reused fixed-size buffer is common and **must** be accepted as
input. That does **not** mean spanning payloads stay zero-copy:

- Complete records are contiguous views (or a copy of that payload only).
- A LEN that straddles chunks is copied (or decoded) when it becomes complete.
- Child `_wire` / `&str` are always one contiguous region.
- A reused stack read buffer cannot be sliced; those bytes are copied into
  long-lived storage at ingest. Prefer one `BytesMut` gather when the source
  is a reused temp buffer. A `Vec<Bytes>` rope is for chunks that are already
  long-lived `Bytes`.

v1 may gather a complete `merge_from` into one `Bytes` **if the scanner loop
is already resumable**. The win we must not throw away is leftover + apply of
complete records, not a public rope type on day one.

### What each field does after a complete record

| Kind | On complete record | On getter |
|---|---|---|
| Singular numerical / enum / bool | Existing `SingularField::merge` (last-wins overwrite) | Eager-like, already materialised |
| Repeated numerical (expanded **and** packed in v1) | Existing `RepeatedField::merge` | `&[T]` from the vec |
| Singular / oneof string / bytes | Store `(offset, len)` into parent `_wire` (not a `Bytes` handle) | Semantic decode; see WireOrSso |
| Nested message (singular / repeated) | Store offset (repeated: list of offsets) | `AddressLazy` / equivalent; **do not** union with full `M` |
| Map | Store entry-LEN offset list | First `attributes()` may build a `HashMap`; no insert during scan |

Unknowns stay in `_wire` (or leftover-adjacent chunks) until a later pass.
Closed-enum unknowns use the existing diversion path when the numerical merge
runs.

### Singular string / bytes: another SSO layer (`WireOrSso`)

Reuse the untagged 3-word slot + common-bit discriminant. Do **not** add a
Rust `enum` tag word.

```
presence   existing EXPLICIT bit (if any)
arm        Wire | Inline | Heap | Failed   (two bits)
slot       3 words, same width as today's SsoBuf
           Wire:   (offset, len) into parent `_wire`
           Inline / Heap: current SsoBuf
           Failed: sticky semantic error (e.g. InvalidUtf8)
```

`bytes::Bytes` does not fit in 3 words. Offsets only.

The new hardness is **promotion on `&self`**: today's SSO arm switch is
`&mut` (`merge` / `_mut`). A lazy getter that returns `&str` must write the
slot on first success. That is `UnsafeCell` / `Cell` on the slot and the arm
bits — separate from the union layout itself.

Do **not** copy this union onto nested messages. Overlaying `M` makes the lazy
slot as wide as eager `M` and kills the size win. Repeated string stays
offset-list / `UnmanagedString` (no SSO today).

After a field promotes to Inline/Heap it no longer depends on `_wire`.
`into_eager` can move already-promoted SSO slots across.

### Implementation reuse (not from scratch, not one dual-mode struct)

| Reuse | New |
|---|---|
| `NumericalType`, tag/varint, `SingularField` / `RepeatedField` for numericals | Resumable record scanner (`leftover`, `NeedMore`) |
| Presence bits, `BitPacked` bool, closed-enum unknown | `WireOrSso` layout + `&self` promote |
| `plan_fields` / `FieldKind` (plugin later) | Map offset list; message offset → `*Lazy` |
| Second emit / aliases (plugin later) | Incremental `push` / `finish` |

Encode on a finished, unmutated lazy message may emit `_wire` (field order
need not match eager). `PartialEq` is semantic, not wire identity. `Clone`
may clone `_wire` and drop unpromoted caches, or clone slots; pick one in
step 8 and test it. Prefer: clone `_wire` + leftover/scan state + already
applied numericals / promoted SSO; unpromoted LEN stays offsets.

## Rejected (do not revive in v1)

- Per-field full rescan with `Uninitialized` / `WireFound` / cursor as the
  primary machine (DESIGN §8 as written).
- Intra-field partial load (repeated iterator that stops mid-scan; public
  `tag_ids_all` as a second surface). First access to a repeated numerical
  field is the whole vec, already filled by the scan.
- Public “is this field loaded?” API.
- Getters before `finish` / EOF.
- Lazy `_mut` / in-place edit of `_wire`.
- Mid-payload splits as a **zero-copy storage contract** for `&str` / child
  `_wire`.
- `TaskView<'buf>`.
- Generator / `TaskImpl<A, L>` before handwritten lazy works.
- Field-level partial/resume inside `protobuf-core` (compose tag/varint
  partial at the puroro layer).

## Implementation steps

One hard piece per step. Each step ends with tests that do not require the
next step. Do not start the plugin until step 9.

### Step 1 — `TaskMessageFallible` on eager `Task` (done 2026-09-06)

Sample only. Getters match inherent shapes wrapped in `Result<_, Infallible>`
(or an associated `Error`). No `_mut` on the trait.

Handwritten: [`sample-generated/src/task/fallible.rs`](../../sample-generated/src/task/fallible.rs).
Test: [`sample-generated/tests/task_fallible.rs`](../../sample-generated/tests/task_fallible.rs).

Done when a generic helper can read `Task` through the trait only.

### Step 2 — Record scanner in `puroro-rt` (done 2026-09-06)

No message type. Input: `Buf` / chunks. Output: complete
`(field_number, wire_type, payload)` or `NeedMore`. Leftover holds an
incomplete tag, length varint, or LEN prefix.

Implemented: [`puroro-rt/src/decode/record.rs`](../../puroro-rt/src/decode/record.rs)
(`RecordScanner` → [`protobuf_core::Field`](../../protobuf-core/src/field.rs)).
`push` returns complete fields; leftover means need more; `finish` is
`TruncatedMessage` if leftover remains.

Tests: mid-varint, mid-fixed, mid-LEN, empty chunk, two chunks that form one
record, trailing garbage after `finish` (error).

Done when the loop is the only parser the later steps call.

### Step 3 — Handwritten lazy shell, numericals only

A sample `TaskLazy` (or a smaller fixture if `Task` is too wide) that:

- feeds the scanner from `merge_from` / internal `push`;
- on a complete **numerical** record, calls existing catalog `merge`;
- skips LEN records (or records them without exposing getters);
- exposes numerical getters only after `finish()`.

Tests: last-wins scalar, implicit zero, packed + expanded mixed, truncated
input, second `merge_from` of a complete message (real protobuf merge).

Done when incremental numerical apply reuses catalog code.

### Step 4 — LEN as offset, decode every get (no union)

Singular string / bytes store `(offset, len)`. Getter decodes from `_wire`
each time. No arm switch, no `UnsafeCell`.

Tests: last-wins string, `InvalidUtf8` before `Optional` is built, presence
vs semantic decode, missing field.

Done when fallible LEN getters are correct without unsafe layout.

### Step 5 — Replace the LEN slot with `WireOrSso`

Only this step may add the union, the extra common bits, and `&self`
promotion. Second get must not re-validate. `Failed` is sticky. `Drop` /
`Clone` honour the live arm.

Done when step 4 tests still pass and promote/drop tests are added.

### Step 6 — Nested messages as offsets → `AddressLazy` / `PointLazy`

Child is a lazy type, not a union with `M`. First child getters may still
use steps 3–4 internally.

Tests: last-wins singular message (merge vs replace — proto merge into the
child once the child exists; first occurrence slices `_wire`, later
occurrence is a second merge into that child **or** last slice wins until
the child is constructed — pick one and document it in this step).
Repeated messages: one slice per occurrence, no merge-into index (same as
eager).

**Lock for step 6:** last-wins singular message keeps the **last** LEN slice
until the child is first constructed; then a later `merge_from` of the
**parent** invalidates / reapplies per merge semantics. Within one parent
scan, only the last occurrence is kept (eager last-wins for the slot; proto
submessage merge applies if multiple occurrences are applied in order
during the same scan). Prefer: **during the scan, merge successive
message LENs into a still-lazy child** (child `push` of each payload) so
behaviour matches eager `merge`. That requires the child scanner to accept
a complete LEN as a finished input. Do that rather than “last slice only”
if both are easy; if not, last-slice-only is acceptable for a first nested
cut and must be called out in the test name.

### Step 7 — Map as offset list

No `HashMap` during scan. First map getter may materialise.

Tests: last-wins per key, missing key/value defaults, empty map.

### Step 8 — `into_eager` and read-only encode

`into_eager` applies remaining LEN promotions / decodes and builds `Task<A>`.
Encode of a finished lazy message may write `_wire` as-is.

Tests: eager-vs-lazy `PartialEq` after `into_eager`; encode length equals
`_wire` len when no extra merge after construction from one buffer.

### Step 9 — `TaskImpl<A, L>` only if needed, then plugin

One emit template, two aliases. Second struct emission from existing
`FieldKind`. Public `push` / chunk API, if not already inherent from step 3,
lands here.

## v1 done / not done

**Done:** sample `TaskLazy` takes chunked or gathered input, `finish`es,
numericals applied on the scan, string/bytes `WireOrSso`, nested + map as
offsets, `TaskMessageFallible` shared with eager.

**Not done:** `TaskView`, lazy mutation, intra-field repeated cursors,
zero-copy spanning LEN, plugin, layout unification, packed-as-unread-slice.

## Pointers

- DESIGN.md §8 — historical; rewrite after step 5 or when aliases land.
- IMPLEMENTATION.md §17 — `TaskLazy` row; point here until runtime exists.
- `sample-generated/` — eager normative output; lazy is added the same way.
- `protobuf-core` — varint/tag partial only; no field-level partial API.
