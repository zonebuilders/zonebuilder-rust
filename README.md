
# zonebuilder

A rust crate for building zones.

Run the CLI:

``` bash
cargo run > circle.geojson
```

    ## bash: /home/robin/.local/share/r-miniconda/envs/r-reticulate/lib/libtinfo.so.6: no version information available (required by bash)
    ## warning: unused variable: `num_segments`
    ##  --> src/lib.rs:5:5
    ##   |
    ## 5 |     num_segments: usize,
    ##   |     ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_num_segments`
    ##   |
    ##   = note: `#[warn(unused_variables)]` on by default
    ## 
    ## warning: unused variable: `boundary`
    ##  --> src/lib.rs:7:5
    ##   |
    ## 7 |     boundary: Option<Polygon<f64>>,
    ##   |     ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_boundary`
    ## 
    ## warning: 2 warnings emitted
    ## 
    ##     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
    ##      Running `target/debug/zonebuilder`

Then read in the GeoJSON file with another tool, e.g.:

``` r
circle = sf::read_sf("circle.geojson")
plot(circle)
```

![](README_files/figure-gfm/unnamed-chunk-2-1.png)<!-- -->

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
