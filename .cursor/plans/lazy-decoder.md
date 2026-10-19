# Plan: Lazy decoder (`TaskLazy`)

**Status (2026-10-12).** Steps 1–9 are done. The sample and the plugin both
emit `Name` / `NameLazy` as `NameImpl<A, Eager>` / `NameImpl<A, Lazy<A>>`.
[DESIGN.md §8](../../DESIGN.md#tasklaya--lazy-parse-timing) describes that
alias. The per-field `FieldCache` machine is rejected; do not revive it.

Updated (2026-10-19): `RecordScanner` emits [`ScannedRecord`](../../puroro-rt/src/decode/record.rs)
/ [`ScannedValue`](../../puroro-rt/src/decode/record.rs). LEN is `payload_len`
only; callers slice `_wire`. Do not rebuild `protobuf_core::Field` with an
owned LEN `AllocVec`. Numerical apply uses catalog `merge_scanned` /
`CopyWirePayload::from_scanned`. Do not rematch `Varint` / `I32` / `I64`
through `merge_scanned_field` (re-encode then catalog `merge`). Packed LEN
uses `payload_len` / `len_span` and `merge_packed_buf` — do not re-read the
length prefix via `merge(WireType::Len)`. Inner bodies were not scanned.

Measured (2026-10-19, Criterion `generated` `task|map`, `taskset -c 0`):
span-only `task_lazy/decode` was 1.295 µs; after `merge_scanned` it is
1.12 µs (~−14%). `map/decode_lazy` did not move (map entries stay spans
at ingest). `merge_packed_buf` did not move typical `task_lazy/decode` or
`map/decode_lazy` (typical Task packed fields are tiny; a noisy first
`task_lazy/decode` run was discarded on recheck). Island-first
`SharedWire` ingest (`push` copies, then scans; leftover is an offset)
did not move typical `task_lazy/decode` (+1.7%) or `map/decode_lazy`
(+0.1%) vs `c51e5da6`. Eager `task/decode` drifted a few percent; treat
gaps under ~10% as noise. Typical TaskLazy decode is about 1.1× eager
after rematch (1.08–1.14 across runs).
Remaining ingest cost is the one island copy of `decode(&[u8])`.
`push(&[u8])` copies onto `SharedWire` first, then scans that instance
(failed scan truncates). `merge_from` / `decode` adopt `Bytes` from
`copy_to_bytes` when the island is empty. Leftover is an offset on the
island. Packed inners still walk the payload (unread packed-as-slice is
out of v1).

Still out of v1: `TaskView`, lazy mutation, `MessageFallible` on the lazy
alias, intra-field repeated cursors, zero-copy spanning LEN, packed repeated
left unread.

Discussion lock (2026-09-06). Sentences below that say "do not introduce
`TaskImpl` yet" or "plugin not done" are the original sequence. The status
line wins.

Hand-written sample first, same as eager. Plugin last.

## Why not a per-field rescan

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
  pub type TaskLazy<A = Global> = TaskImpl<A, Lazy<A>>;
  ```

  Do **not** introduce `TaskImpl` until both handwritten types exist and
  duplication hurts (step 9). Until then the sample types are `Task` and
  `TaskLazy`.

- Shared generic read surface: `task::MessageFallible` (`Error = Infallible` on
  eager, `DecodeError` on lazy). Eager inherent `_mut` stays off the trait.
- Lazy is **read-oriented**. Mutation is `into_eager(self) -> Result<Task<A>,
  DecodeError>` (and then the existing mut API). No lazy `_mut` in v1.

### Nested child parse timing (2026-09-24)

Default: the child follows the parent layout. Eager parent stores an eager
child; lazy parent stores a lazy child. Callers still pick the parent by
using `Task` or `TaskLazy`. The option does not collapse those two aliases
into one.

Override with `EAGER` / `LAZY` / `UNSPECIFIED`:

1. Field option `(puroro.child_parse)`, when set.
2. Otherwise the enclosing message option `(puroro.message_child_parse)`,
   when set (default for fields declared on that message, not for every use
   of that type elsewhere). Extension names are unique in a package, so the
   message option cannot reuse `child_parse`. Both use field number `51404`.
3. Otherwise follow the parent layout.

Applies to singular messages, repeated messages, map values, and oneof
message variants. Orthogonal to `(puroro.message_layout)` (inline vs boxed).
A pin that differs from the parent is a concrete child type on both aliases.
A follow-parent field is a `Layout` associated type.

Sample: `origin` / `watchers` / `postal` are step 3. `assignee` is a
hand-written field-level `LAZY` pin (`AddressLazy` even on eager `Task`,
`set_assignee` goes through `into_lazy`). Do not emit that pin unless the
option says so.

`ChildParse` is in `options.proto`. The plugin reads field number `51404`
on both `FieldOptions` (`child_parse`) and `MessageOptions`
(`message_child_parse`).

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

- Complete records are metadata (and a small numeric). LEN is a span into
  the island, not a second payload copy.
- A LEN that straddles chunks is **not** copied into the scan record.
  Leftover holds the unfinished tail; that prefix is already on the island
  from the earlier append, then the completing chunk is appended.
- Child `_wire` / `&str` are always one contiguous region on the island.
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

### Step 1 — `task::MessageFallible` on eager `Task` (done 2026-09-06)

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
(`RecordScanner` → `ScannedRecord` / `ScannedValue`). `push` returns complete
records; leftover means need more; `finish` is `TruncatedMessage` if leftover
remains. LEN payloads are not copied into the scan record
(updated 2026-10-19; was `protobuf_core::Field` + owned LEN).

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

Done (2026-09-06): `TaskLazy` + `RecordScanner` + catalog `merge` via
`merge_scanned_field`. LEN / map / nested / oneof skipped. Getters require
`finish` (`UnfinishedMessage` / `TruncatedMessage`).

### Step 4 — LEN as offset, decode every get (no union)

Singular string / bytes store `(offset, len)`. Getter decodes from `_wire`
each time. No arm switch, no `UnsafeCell`.

Tests: last-wins string, `InvalidUtf8` before `Optional` is built, presence
vs semantic decode, missing field.

Done (2026-09-06): singular `title` / `owner_id` / `payload` store `WireSpan`
into `_wire`; getter decodes every time (`InvalidUtf8` before `Optional`).
`has_*` is presence-only. No union / `UnsafeCell`.

### Step 5 — Replace the LEN slot with `WireOrSso`

Only this step may add the union, the extra common bits, and `&self`
promotion. Second get must not re-validate. `Failed` is sticky. `Drop` /
`Clone` honour the live arm.

Done (2026-09-06): `WireOrSsoSlot` + two common arm bits + `&self`
promotion (`InteriorBitArray`). Second get does not re-validate. `Failed`
is sticky. `Drop` / `clone_in` honour the live arm.

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

Done (2026-09-06): `AddressLazy` / `PointLazy`. During the parent scan each
complete message LEN is merged into a still-lazy child (proto submessage
merge, not last-slice-only). Repeated `watchers` appends one child per
occurrence.

Updated (2026-09-07): nested children `adopt` the island-root
[`SharedWire`](../../puroro-rt/src/decode/shared_wire.rs) (v1: `Bytes` +
stored `A`) and record payload spans. No second copy of the LEN body.

### Step 7 — Map as offset list

No `HashMap` during scan. First map getter may materialise.

Tests: last-wins per key, missing key/value defaults, empty map.

Done (2026-09-06): `MapSpans` stores `WireSpan`s during the parent scan
and builds the ready map on the first `attributes()`. A later parent
`merge_from` after that get merges the new entry into the ready map.
Missing key / value use proto defaults (`""` / `0`). Empty map is empty.

### Step 8 — `into_eager` and read-only encode

`into_eager` applies remaining LEN promotions / decodes and builds `Task<A>`.
Encode of a finished lazy message may write `_wire` as-is.

Tests: eager-vs-lazy `PartialEq` after `into_eager`; encode length equals
`_wire` len when no extra merge after construction from one buffer.

Done (2026-09-06): `into_eager` merges `_wire` into eager `Task` / `Address` /
`Point`. Finished encode writes
`_wire` as-is. After two parent `merge_from`s, encode length is the
concatenated ingest; `into_eager` matches eager merge of the same bodies.

### Step 9 — `TaskImpl<A, L>` only if needed, then plugin

One emit template, two aliases. Second struct emission from existing
`FieldKind`. Public `push` / chunk API, if not already inherent from step 3,
lands here.

Done (2026-10-12): sample `TaskImpl<A, L>` and the plugin's `NameImpl<A, L>`.
`push` / `finish` are inherent on the lazy alias. Oneof and repeated string /
bytes are included. `Layout` associated types cover slots that differ.

## v1 done / not done

**Done:** sample and plugin lazy aliases. Chunked or gathered input,
`finish`, numericals applied on the scan, string / bytes `WireOrSso`, nested
messages and maps as spans, repeated `labels` as a span list (first get
materialises), oneof, `Layout` associated types. The scan record itself is
a span (`ScannedRecord` / `ScannedValue`); complete LEN is not copied off
the island. Numericals use catalog `merge_scanned` (no rematch). Packed LEN
uses `merge_packed_buf`. `push` copies onto `SharedWire` first; leftover is
an island offset. Eager `Task` implements handwritten `task::MessageFallible`
(`Error = Infallible`).

**Not done:** `TaskView`, lazy mutation, `MessageFallible` for `TaskLazy`
(follow-parent fields change type with the parent; the plugin does not emit
the trait), intra-field repeated cursors, zero-copy spanning LEN,
packed-as-unread-slice. Lazy `unknown_fields()` is empty; unknown tags stay
in the wire and surface on `into_eager`.

## Revisit later — VERIFY string vs closed-enum unknown

`utf8_validation=VERIFY` failures are **not** locked to the closed-enum
`UnknownClosedEnum` → `unknown_fields` path.

v1 lazy `labels` (and eager repeated `string` merge) treat `InvalidUtf8` as
a getter / merge failure for that materialise. Per-occurrence divert — skip
the bad LEN, keep valid elements, park the raw record — should be cheap to
add later on the materialise / `RepeatedElement` merge path. Do not treat
the current policy as final. Same question applies to singular lazy
`Failed` vs diverting `title` into unknowns.

## Pointers

- DESIGN.md §8 — shipped lazy alias. This note keeps the lock and the rejected list.
- IMPLEMENTATION.md §3 / §17 — plugin status. The runtime exists.
- `sample-generated/` — eager normative output; lazy is added the same way.
- `protobuf-core` — varint/tag partial only; no field-level partial API.
