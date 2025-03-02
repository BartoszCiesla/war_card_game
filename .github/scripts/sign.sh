#!/usr/bin/env bash

set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "❌ Error: Missing required arguments!"
    echo "Usage: $0 \"<list of Cargo.toml files>\" \"<list of files to sign>\""
    echo "Example: $0 \"Cargo.toml crate/Cargo.toml\" \"package.tgz package.zip\""
    exit 1
fi

readonly CARGO_FILES=(${1})
readonly FILES_TO_SIGN=(${2})

if [ "${#CARGO_FILES[@]}" -eq 0 ]; then
    echo "⚠️ Warning: The list of Cargo.toml files is empty!"
fi

if [ "${#FILES_TO_SIGN[@]}" -eq 0 ]; then
    echo "⚠️ Warning: The list of other files is empty!"
fi

# Generate a new key pair
rsign generate -f -W -p minisign.pub -s minisign.key

# Mask the private key
echo "::add-mask::$(tail -n1 minisign.key)"

if [[ -n "${GITHUB_REPOSITORY:-}" ]] && [[ -n "${GITHUB_SHA}" ]] && [[ -n "${GITHUB_RUN_ID}" ]]; then
    readonly GITHUB_PREFIX="${GITHUB_REPOSITORY}@${GITHUB_SHA} (run ${GITHUB_RUN_ID})"
else
    readonly GITHUB_PREFIX="local-signing"
fi

TRUSTED_COMMENT="Trusted by ${GITHUB_PREFIX} $(date +'%Y-%m-%d %H-%M-%S')"
readonly TRUSTED_COMMENT

for file in "${FILES_TO_SIGN[@]}"; do
    if [[ -f "$file" ]]; then
        echo "✅ Signing: ${file}"
        rsign sign -W -s minisign.key -t "${TRUSTED_COMMENT}" -x "${file}.sig" "${file}"
    else
        echo "❌ Not found: ${file}"
    fi
done

# Append the public key to evey Cargo.toml file
for cargo_file in "${CARGO_FILES[@]}"; do
    if [[ -f "${cargo_file}" ]]; then
        echo "✅ Adding public key: ${cargo_file}"
        cat >> "${cargo_file}" <<EOF

[package.metadata.binstall.signing]
algorithm = "minisign"
pubkey = "$(tail -n1 minisign.pub)"
EOF
    else
        echo "❌ Not found: ${cargo_file}"
    fi
done

# Remove private key
rm minisign.key

echo "🎉 Artifacts signing completed!"
