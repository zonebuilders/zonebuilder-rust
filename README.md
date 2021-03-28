
# zonebuilder

A rust crate for building zones.

Run the CLI:

``` bash
cargo run > circle.geojson
```

    ##     Finished dev [unoptimized + debuginfo] target(s) in 0.02s
    ##      Running `target/debug/zonebuilder`

Take a look at the output:

``` bash
head -c 80 circle.geojson
```

    ## {"coordinates":[[[1.0,0.0],[0.866025,0.499999],[0.5,0.866025],[0.0,1.0],[-0.4999

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
