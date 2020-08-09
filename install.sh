#!/usr/bin/env bash
set -euo pipefail
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

pushd "${DIR}"
echo "building chronocam release"
cargo build --release
popd

# this script currently assumes you're running on a raspberry pi
# with raspbian running
echo "installing chronocam release"
cp "${DIR}/target/release/chronocam" /usr/local/bin

echo "installing scripts"
cp "${DIR}/deploy/scripts/*.sh" /usr/local/bin

echo "installing systemd units"
cp "${DIR}"/deploy/systemd/*.service /etc/systemd/system/
cp "${DIR}"/deploy/systemd/*.timer /etc/systemd/system/

echo "starting units"
systemctl enable chronocam.service
systemctl start chronocam.service

systemctl enable systemctl start chronocam.service.timer