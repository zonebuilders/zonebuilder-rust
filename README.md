
# zonebuilder

A rust crate for building zones.

Run the CLI:

``` bash
cargo run > circle.geojson
```

    ##    Compiling zonebuilder v0.1.0 (/mnt/57982e2a-2874-4246-a6fe-115c199bc6bd/orgs/zonebuilders/zonebuilder-rust)
    ## warning: unused variable: `boundary`
    ##   --> src/lib.rs:27:5
    ##    |
    ## 27 |     boundary: Option<Polygon<f64>>,
    ##    |     ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_boundary`
    ##    |
    ##    = note: `#[warn(unused_variables)]` on by default
    ## 
    ## warning: 1 warning emitted
    ## 
    ##     Finished dev [unoptimized + debuginfo] target(s) in 0.63s
    ##      Running `target/debug/zonebuilder`

Take a look at the output:

``` bash
head -c 80 circle.geojson
```

    ## {"coordinates":[[[1.0,0.0],[0.998652088398823,0.05190381813189974],[0.9946119873

Then read in the GeoJSON file with another tool, e.g. R:

``` r
circle = sf::read_sf("circle.geojson")
plot(circle)
```

![](README_files/figure-gfm/circle-1.png)<!-- -->

``` r
file.remove("circle.geojson")
```

    ## [1] TRUE

<!--
The crate template was made with the following command:

```bash
cargo new --lib zonebuilder
```

```bash
mv -v zonebuilder/* .               
# renamed 'zonebuilder/Cargo.toml' -> './Cargo.toml'
# renamed 'zonebuilder/src' -> './src'
```

Edit the .rs files in src folder.

Then run:

```bash
cargo test
```

-->
