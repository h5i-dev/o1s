#!/usr/bin/env bash
# Run the server locally. Storage is the local log in $O1S_DATA unless
# O1S_DATABASE_URL is set; `scripts/dev.sh --postgres` starts a local
# PostgreSQL container (scripts/postgres.sh) and uses it.
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ "${1:-}" == "--postgres" ]]; then
    export O1S_DATABASE_URL="$(scripts/postgres.sh)"
fi
export O1S_DATA="${O1S_DATA:-.o1s}"
mkdir -p "$O1S_DATA"
if [[ ! -f "$O1S_DATA/secret" ]]; then
    (umask 077; python3 -c 'import secrets; print(secrets.token_hex(32))' > "$O1S_DATA/secret")
fi
export H5I_APP_SECRET="${H5I_APP_SECRET:-$(cat "$O1S_DATA/secret")}"
cargo build -q --release -p o1s-server
server="${CARGO_TARGET_DIR:-target}/release/o1s-server"
"$server" token 1 1
printf '\nAbove: author token. Reviewer token:\n'
"$server" token 1 2
printf '\nCheck reporter token:\n'
"$server" token 1 3
printf '\nOpen http://%s/#/repos and paste a token.\n' "${O1S_BIND:-127.0.0.1:3000}"
exec "$server"
