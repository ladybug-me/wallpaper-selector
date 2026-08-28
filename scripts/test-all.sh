#!/bin/sh
set -eu

cd "$(dirname "$0")/.."

usage() {
    printf '%s\n' 'test-all.sh [--live] [--keep-going]'
}

LIVE=0
KEEP_GOING=0
for argument in "$@"; do
    case "$argument" in
        --live) LIVE=1 ;;
        --keep-going) KEEP_GOING=1 ;;
        -h|--help) usage; exit 0 ;;
        *) usage >&2; exit 2 ;;
    esac
done

VERIFY_ROOT="${SKWD_VERIFY_ROOT:-../skwd-verify}"
if [ ! -f "$VERIFY_ROOT/scripts/python_suite.py" ]; then
    echo "missing skwd-verify checkout at $VERIFY_ROOT (set SKWD_VERIFY_ROOT)" >&2
    exit 1
fi

status=0
run() {
    label=$1
    shift
    printf '\n== %s ==\n' "$label"
    if "$@"; then
        return 0
    else
        code=$?
    fi
    status=$code
    if [ "$KEEP_GOING" -eq 0 ]; then
        exit "$code"
    fi
}

run format cargo fmt --check
run clippy cargo clippy --locked --all-targets -- -D warnings
run tests cargo test --locked --release
run allocations cargo test --locked --release --features obs-heap alloc_free -- --test-threads=1
run python env PYTHONDONTWRITEBYTECODE=1 python3 "$VERIFY_ROOT/scripts/python_suite.py"
run unsafe-inventory "$VERIFY_ROOT/scripts/check-unsafe-code.sh"
run release cargo build --locked --release --package skwd-wall --bin skwd-wall

if [ "$LIVE" -eq 1 ]; then
    echo "live E2E moved: run skwd-verify wall.* suites and skwd-deck scripts/test-e2e.sh" >&2
    status=1
fi

exit "$status"
