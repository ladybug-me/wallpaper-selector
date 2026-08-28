#!/bin/sh
set -eu
cd "$(dirname "$0")/.."

if ! command -v cargo-mutants >/dev/null 2>&1; then
    echo "cargo-mutants not installed. Install with: cargo install cargo-mutants" >&2
    exit 2
fi

if [ "${1:-}" = "--all" ]; then
    exec cargo mutants --workspace
fi

if [ "$#" -gt 0 ]; then
    files="$*"
else
    files="src/domain/schedule/blocks.rs
src/frontend/animation/animation.rs
src/domain/library/catalog/catalog.rs
src/domain/library/filter/filter.rs
src/domain/library/search/search.rs
src/backend/picker/library/state.rs"
fi

set --
for target in $files; do
    set -- "$@" --file "$target"
done
exec cargo mutants --workspace "$@"
