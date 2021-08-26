#!/bin/bash

set -eu

BEVY_BUILDER_CONTAINER="bevy-builder"

containerId=$(podman create "$BEVY_BUILDER_CONTAINER")
podman cp "main.rs" "$containerId:/project/src/main.rs"
podman start --attach --interactive "$containerId"
podman cp "$containerId:/project/out/." "build/"
podman rm "$containerId" >/dev/null