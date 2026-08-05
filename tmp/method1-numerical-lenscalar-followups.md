# Follow-ups after Method 1 (`Numerical` / `LenScalar`) refactor

Scratch note for future agents. Safe to delete once addressed or dismissed.
Context: Qiita-style Method 1 — blankets on `Numerical<C>` / `LenScalar<C>`, public aliases `ProtoInt32` / `ProtoString` / ….

## Optional cleanups (non-blocking)

1. **Codec trait naming asymmetry**
   - LEN: `LenCodec` + `LenScalar<C>`
   - Numeric: `NumericalType` + `Numerical<C>` (`NumericalType` is the codec, despite the old name)
   - Consider renaming `NumericalType` → `NumericalCodec` (or similar) for parallel naming. Touch docs/comments that still say “marker implements NumericalType”.

2. **Module layout**
   - Concrete aliases / codecs live mainly in `puroro-rt/src/fields/wire/numerical.rs` and `len.rs`.
   - `varint.rs` / `fixed.rs` mostly re-export aliases; enum kind traits (`Open` / `Closed` / `OpenEnum` / `ClosedEnum`) stay in `varint.rs`.
   - Optional: flatten re-exports or document the split in `wire.rs` / `IMPLEMENTATION.md` so readers don’t hunt unit structs in `varint.rs`.

3. **Diagnostics / rustdoc**
   - Errors and rustdoc may show `Numerical<Int32Codec>` / `LenScalar<StringCodec>` instead of `ProtoInt32` / `ProtoString`.
   - No action required unless this becomes noisy; aliases are intentional for the public codegen surface.

4. **Docs drift**
   - `IMPLEMENTATION.md` / `DESIGN.md` may still describe pre-refactor marker layout (bare ZST + `impl NumericalType for ProtoInt32`). Update when touching those docs anyway.

5. **Out of scope (do not “complete” this refactor by doing these)**
   - Property-tuple Proto types, `is-bool` tags, folding `ProtoMessage` into `LenScalar`.
   - Crate-root re-export of `Numerical` / `LenScalar` / codecs — intentionally omitted; keep codegen on `Proto*` aliases only.

## Done / keep as-is

- `ProtoBool` singular stays special (`Slot = ()` + `BitPacked`); repeated uses numerical blankets.
- Visibility: `Numerical` / `LenScalar` / codecs / `LenCodec` / `NumericalType` are `pub` for coherence but **not** crate-root re-exported.
