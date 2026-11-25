#!/usr/bin/env bash
# Refresh vendored google.protobuf .proto files from a pinned protobuf release.
#
# Usage:
#   ./puroro-types/scripts/vendor-google-protos.sh           # tag from proto/UPSTREAM
#   ./puroro-types/scripts/vendor-google-protos.sh v32.1     # override tag
#
# Writes files under puroro-types/proto/ and updates proto/UPSTREAM.
# Requires: curl, python3. Network access to github.com.

set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
CRATE_DIR=$(cd -- "${SCRIPT_DIR}/.." && pwd)
PROTO_DIR="${CRATE_DIR}/proto"
DEST_DIR="${PROTO_DIR}/google/protobuf"
UPSTREAM_FILE="${PROTO_DIR}/UPSTREAM"
REPO=https://github.com/protocolbuffers/protobuf
RAW_BASE=https://raw.githubusercontent.com/protocolbuffers/protobuf

FILES=(
  any.proto
  descriptor.proto
  duration.proto
  empty.proto
  field_mask.proto
  struct.proto
  timestamp.proto
  wrappers.proto
)

read_upstream_tag() {
  if [[ -f "${UPSTREAM_FILE}" ]]; then
    # shellcheck disable=SC2002
    awk -F= '/^tag=/{print $2; exit}' "${UPSTREAM_FILE}"
  fi
}

TAG=${1:-$(read_upstream_tag)}
if [[ -z "${TAG}" ]]; then
  echo "error: no tag argument and no tag= in ${UPSTREAM_FILE}" >&2
  exit 1
fi

resolve_commit() {
  local tag=$1
  python3 - "${tag}" <<'PY'
import json, sys, urllib.request

tag = sys.argv[1]
ref = json.load(
    urllib.request.urlopen(
        f"https://api.github.com/repos/protocolbuffers/protobuf/git/refs/tags/{tag}"
    )
)
obj = ref["object"]
if obj["type"] == "tag":
    tag_obj = json.load(urllib.request.urlopen(obj["url"]))
    print(tag_obj["object"]["sha"])
else:
    print(obj["sha"])
PY
}

echo "Vendoring ${REPO} @ ${TAG} …"
COMMIT=$(resolve_commit "${TAG}")
echo "Resolved commit ${COMMIT}"

mkdir -p "${DEST_DIR}"
TMP=$(mktemp -d)
trap 'rm -rf "${TMP}"' EXIT

curl -fsSL "${RAW_BASE}/${TAG}/LICENSE" -o "${TMP}/LICENSE"
for f in "${FILES[@]}"; do
  curl -fsSL "${RAW_BASE}/${TAG}/src/google/protobuf/${f}" -o "${TMP}/${f}"
done

# Replace only after every download succeeds.
cp "${TMP}/LICENSE" "${PROTO_DIR}/LICENSE"
for f in "${FILES[@]}"; do
  cp "${TMP}/${f}" "${DEST_DIR}/${f}"
done

{
  cat <<EOF
# Pinned Protocol Buffers sources for puroro-types/proto/.
#
# Refresh with:
#   ./puroro-types/scripts/vendor-google-protos.sh
# Or pin another release tag:
#   ./puroro-types/scripts/vendor-google-protos.sh v32.1
#
# Do not edit the .proto files by hand; re-run the script so this pin stays accurate.

repo=${REPO}
tag=${TAG}
commit=${COMMIT}
# Paths are relative to the upstream repo root.
src_root=src/google/protobuf
license=LICENSE

files=
EOF
  for f in "${FILES[@]}"; do
    printf '  %s\n' "${f}"
  done
} >"${UPSTREAM_FILE}"

echo "Updated ${#FILES[@]} protos + LICENSE under ${PROTO_DIR}"
echo "Pin written to ${UPSTREAM_FILE}"
