# Service / RPC stub codegen (tonic)

Plan for generating gRPC client/server stubs from `service` / `rpc` definitions,
wired to the existing `puroro::tonic::PuroroCodec` bridge.

Status: design only. No code yet.

## Goal

Given a `.proto` with services, `protoc-gen-puroro` emits tonic-compatible
client and server stubs that encode/decode request and response messages with
`PuroroCodec`, so users can implement and call gRPC without prost.

## Non-goals (v1)

- HTTP / gRPC-Gateway (`google.api.http`) annotations.
- A second RPC runtime (e.g. `grpcio`, tower-only without tonic).
- Lazy message types (`NameLazy`) as RPC request/response types.
- Custom-allocator decode on the wire path (stubs use `Message::decode` →
  `Default` → `Global`, same constraint as today's `PuroroCodec`).
- Reflection / gRPC server reflection from generated stubs.
- Changing the public `PuroroCodec` API (unless a concrete stub gap forces it).

## Current state

| Piece | Status |
|---|---|
| `puroro` feature `tonic` / `PuroroCodec` | Shipped — message byte codec only |
| `FileDescriptorProto.service` (field 6) in plugin IR | Skipped in `decode_file_descriptor` |
| Resolve / FieldKind / emit for services | Absent |
| DESIGN / README future-work bullet | Documents the gap |

Message types already default `A = Global`, so stub signatures can name
`HelloRequest` without an explicit allocator parameter.

## Recommended approach

Reuse **`tonic-build`'s `CodeGenBuilder`** from inside `protoc-gen-puroro`.

1. Decode services into the descriptor IR.
2. Resolve each method's `input_type` / `output_type` to generated message paths.
3. Implement `tonic_build::{Service, Method}` for that resolved IR.
4. Call `CodeGenBuilder::generate_client` / `generate_server` with
   `codec_path = "::puroro::tonic::PuroroCodec"`.
5. Install the resulting items into the package module (tonic's usual
   `{service}_client` / `{service}_server` modules).
6. Extend the generated `Cargo.toml` so stubs compile behind a crate feature.

Why this instead of hand-rolling templates:

- tonic already exposes `Service` / `Method` for non-prost backends.
- Client/server surface (unary + three streaming shapes, interceptors,
  compression knobs, `connect`) tracks tonic upgrades in one place.
- `PuroroCodec` already implements `Default` + `Codec`, which is what the
  generated `let codec = Path::default()` expects.

Accept that tonic-build output uses tonic's style (`use tonic::codegen::*`,
no leading `::` on every path). Treat those modules as sealed stub crates
inside the forest; do not rewrite them to match message-codegen path rules.

### Alternatives set aside

| Alternative | Why not for v1 |
|---|---|
| Hand-rolled stub templates in `emit/` | Duplicates tonic-build; breaks when tonic changes |
| Separate `tonic-puroro-build` crate users call after message gen | Worse UX; puroro's primary path is already the protoc plugin |
| Generate messages here, leave users on prost stubs | Defeats the point of `PuroroCodec` |

## Public surface

### Module layout

For `package helloworld; service Greeter { rpc SayHello(HelloRequest) returns (HelloReply); }`:

```text
helloworld::HelloRequest
helloworld::HelloReply
helloworld::greeter_client::GreeterClient
helloworld::greeter_server::{Greeter, GreeterServer}
```

Same placement tonic-prost uses: stub modules sit beside message types in the
package module. Naming: `to_snake_case(ServiceName) + "_client"/"_server"`.

Collision note: a message companion `greeter` and stub mods `greeter_client` /
`greeter_server` do not clash. Document that a proto entity must not claim those
suffix module names (or leave clashes to `rustc`, consistent with today's merge
rules).

### Message types in stubs

Eager aliases with default `Global` only:

```rust
// conceptually
type Request = self::_root::helloworld::HelloRequest; // = HelloRequest<Global>
```

`request_response_name` returns forest-qualified paths (`self::_root::…`), not
`crate::` / leading `::` host paths, so embedding the forest still works.

### Generated crate feature

Mirror `descriptors`:

```toml
[dependencies]
puroro = { version = "…", features = ["tonic"] }  # when services emitted; or optional — see below
tonic = { version = "0.14", default-features = false, features = ["codegen", "transport"] }

[features]
default = []
descriptors = []
tonic = []   # cfg-gates greeter_client / greeter_server modules
```

Preferred wiring (aligned with how `puroro`'s own `tonic` feature works):

- When at least one service is emitted, the generated manifest lists `tonic` as
  an **optional** dependency and enables `puroro/tonic` through the generated
  crate feature `tonic`.
- Stub modules are wrapped in `#[cfg(feature = "tonic")]`.
- Message-only builds stay free of a tonic requirement.

Pin the `tonic` version from `protoc-gen-puroro/build.rs` the same way as
`puroro` / `puroro-rt` (add `tonic` as a build-dependency solely for lockfile
version capture, or read it from the plugin's direct `tonic-build` → `tonic`
edge — pick one mechanism and document it).

### Plugin options (`--puroro_opt`)

| Option | Default | Meaning |
|---|---|---|
| *(services present)* | emit cfg-gated stubs | No flag required when the file has services |
| `tonic=false` | — | Suppress stub emission even if services exist |
| `tonic_client=false` | client on | Skip client modules |
| `tonic_server=false` | server on | Skip server modules |

Rationale for auto-emit + feature gate: users with services expect stubs; users
who only want messages keep `default = []` and never enable `tonic`.

### RPC shapes in v1

All four: unary, server-streaming, client-streaming, bidi. Cost is almost only
the `Method::{client,server}_streaming` flags once `tonic-build` is wired.

## Implementation phases

### Phase 0 — Spec lock

- Move this plan's public decisions into `DESIGN.md` (new § under generated
  code / future work demotion).
- Update `README.md` / `IMPLEMENTATION.md` status rows once Phase 1 starts;
  keep the future-work bullet until stubs actually ship.
- Record naming, feature gate, and `codec_path` as normative.

### Phase 1 — Descriptor IR

- Decode `FileDescriptorProto.service = 6` → `ServiceDesc`.
- Decode `ServiceDescriptorProto` / `MethodDescriptorProto` (name, input_type,
  output_type, client_streaming, server_streaming; options/deprecated as needed).
- Unit tests on synthetic wire / fixture bytes (no full protoc required).

### Phase 2 — Resolve

- Attach services to `resolved::File`.
- Resolve method I/O to `TypeRef` / message FQNs; fail codegen if a method
  references a non-message or missing type.
- Snake / Pascal ident helpers for service and method names (reuse
  `case` / `escape_ident`; extend allowed-names docs for service/rpc idents).

### Phase 3 — Emit via tonic-build

- Add `tonic-build` dependency to `protoc-gen-puroro` (same major as
  `puroro`'s `tonic` 0.14).
- Adapter types implementing `tonic_build::{Service, Method}`.
- `codec_path`: `::puroro::tonic::PuroroCodec`.
- `CodeGenBuilder` with `build_transport(true)` by default; map
  `tonic_client` / `tonic_server` opts.
- Parse generated `TokenStream` into `syn::Item`s and install under the package
  module; wrap with `#[cfg(feature = "tonic")]`.
- Extend generated `Cargo.toml` writer + `build.rs` version capture for tonic.

### Phase 4 — Tests and sample

- New `puroro-codegen-tests` fixture (e.g. `grpc_helloworld`): compile stubs with
  `--features tonic`.
- Cover unary + at least one streaming method in the fixture proto.
- Optional: thin hand-written mirror in `sample-generated` only if it helps
  review; prefer the live plugin fixture as source of truth.
- Runtime smoke (optional follow-up): in-process tonic server/client round-trip
  in that fixture or a small example — not required to land Phase 3.

### Phase 5 — Docs and polish

- Plugin README option table; root README future-work → done note.
- `AGENTS.md` test commands for the tonic feature on codegen fixtures.
- Deprecated rpc / service comments if cheap via tonic-build hooks.

## Touched components

| Component | Change |
|---|---|
| `protoc-gen-puroro` | IR, resolve, emit adapter, Cargo.toml writer, opts |
| Generated crate manifest | optional `tonic` feature + deps when services exist |
| `puroro` / `PuroroCodec` | Likely unchanged |
| `puroro-rt` | Unchanged |
| `puroro-codegen-tests` | New fixture |
| DESIGN / README / IMPLEMENTATION / AGENTS | Spec + status |

## Risks

- **tonic-build style vs puroro path rules.** Accept sealed stub modules; do not
  force `_root` rewriting inside tonic templates.
- **Version skew.** Plugin's `tonic-build` major must stay aligned with the
  `tonic` version written into generated manifests and with `puroro`'s optional
  `tonic` dependency.
- **WKT request/response types** (e.g. `google.protobuf.Empty`). Same as today
  for messages: the type must be in the generate set or another generated crate
  the user depends on. Call out in docs; do not special-case in v1.
- **Plugin binary size / deps.** `tonic-build` pulls quote/syn/prost-adjacent
  codegen crates; acceptable for a codegen binary.

## Open questions (decide before Phase 3)

1. **Auto-emit vs explicit `tonic=true`.** Plan recommends auto-emit +
   generated-crate feature gate. Explicit opt-in is the conservative alternative
   if pulling optional `tonic` into every service-bearing manifest is unwanted.
2. **`build_transport`.** Default on (needs `tonic/transport` in the generated
   dep). Off reduces deps but drops `connect()`.
3. **In-process round-trip test in v1** or compile-only fixture first.

## Success criteria

- A helloworld-style `.proto` produces compiling `*_client` / `*_server` modules
  when the generated crate enables `tonic`.
- Handlers use puroro message types; bytes go through `PuroroCodec`.
- Message-only crates and service crates with `tonic` feature off still build
  without a tonic dependency requirement at compile time of default features.
