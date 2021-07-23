# Zone Builder web demo

This is a demonstration of gluing a [Leaflet](https://leafletjs.com/)
Javascript web map to the Rust zonebuilder crate. You can use this to draw a
clockboard and download the GeoJSON.

## Building

These instructions will be improved soon. I think using npm probably simplifies
things, and we can hook up automatic deployment to Github pages.

But for now:

1.  Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) if you haven't before, e.g. with

```bash
 curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
```

2.  Compile the Rust code to WebAssembly. Run this from the root directory in
    this repo: `cd ..; wasm-pack build --target web -- --features wasm`

3.  Back in this directory, run `./serve_locally.py`

4.  Open http://0.0.0.0:8000/ in your browser.
