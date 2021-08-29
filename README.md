![demo of the website](./docs/bevy-playground.mp4)

### Building

```sh
cd bevy-builder
podman build . --tag bevy-builder
```

```sh
npm install -g parcel@next
cd bevy-playground-website
parcel build index.html
```

```sh
cd bevy-playground-server
cargo run --release
```