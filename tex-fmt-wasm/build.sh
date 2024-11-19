#!/usr/bin/env bash

# wasm-pack needs to be configured for web or nodejs so we should
# probably update this to generate two separate packages, one for
# the browser, and one for node.js
wasm-pack build --debug --target web
