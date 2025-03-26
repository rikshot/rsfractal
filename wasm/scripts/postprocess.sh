#!/usr/bin/env bash -xue

if [ $TRUNK_PROFILE == "release" ]; then
    gzip --best --keep "$TRUNK_STAGING_DIR"/*{.js,.wasm}
    brotli --best --keep "$TRUNK_STAGING_DIR"/*{.js,.wasm}
fi
