#! /bin/bash

set -o errexit
set -o nounset

cargo build --release

BINS_DIR="${CARGO_TARGET_DIR:-target}/release"
INSTALL_DIR='/usr/local/bin'
declare -a BINS

for module in $(find . -maxdepth 2 -mindepth 2 -type f -name Cargo.toml); do
    bin="$(basename "${module%/Cargo.toml}")"
    BINS+=("${BINS_DIR}/${bin}")
done

sudo cp "${BINS[@]}" "${INSTALL_DIR}"
sudo setcap cap_sys_ptrace=eip "${INSTALL_DIR}/need-restart" || true

exit $?
