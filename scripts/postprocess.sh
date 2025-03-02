#!/usr/bin/env bash

gzip --best --keep "$TRUNK_STAGING_DIR"/*{.js,.wasm}
brotli --best --keep "$TRUNK_STAGING_DIR"/*{.js,.wasm}
