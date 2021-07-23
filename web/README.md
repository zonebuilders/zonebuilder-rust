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

2.  Compile the Rust code to WebAssembly. Run this from the root directory of the `zonebuilder-rust` project: 

```bash
wasm-pack build --target web -- --features wasm
```

After a few seconds compiling, you should see a message saying something like:

```bash
# [INFO]: :-) Done in 30.20s
# [INFO]: :-) Your wasm pkg is ready to publish at /home/robin/orgs/zonebuilders/zonebuilder-rust/pkg.
```

If so, it worked!


3.  Set your working directory to the `web` folder and serve the map as follows:

```bash
cd web
./serve_locally.py
```

4.  Open http://0.0.0.0:8000/ in your browser. You should see web-based graphical user interface for generating zoning systems.

