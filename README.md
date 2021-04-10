
# zonebuilder

A rust crate for building zones.

It is an experimental and in-progress project to implement the
functionality in the
[`zonebuilder`](https://zonebuilders.github.io/zonebuilder/) R package
in the systems programming language Rust.

Why?

-   It should eventually enable more people to benefit from free and
    open source software for creating zoning systems because Rust
    enables the creation of binaries for Windows, Mac and the free and
    open source Linux operating system on which the package was
    originally developed (Rust can also compile to
    [WASM](https://webassembly.org/) enabling complex applications such
    as [A/B Street](https://github.com/a-b-street/abstreet) to run in
    browser — the thinking being if that can run in browser surely as
    simple application to build zones can!)
-   Computational efficiency: the process of building zones is not
    particularly computationally intensive but this Rust crate may
    eventually be fast and quick to install and use, possibly from
    higher level languages such as R using Rust interfaces such as
    [`extendr`](https://github.com/extendr/extendr)
-   For fun and education: as a simple crate it serves as a good way to
    show how Rust code is organised and how it works

To reproduce the example shown here you need to have the rust toolchain
installed.

Assuming you do, you can run the code as follows

### Clone the repo

``` bash
git clone https://github.com/zonebuilders/zonebuilder-rust.git
cd zonebuilder-rust
git checkout circles
```

    ## Cloning into 'zonebuilder-rust'...
    ## Switched to a new branch 'circles'
    ## Branch 'circles' set up to track remote branch 'circles' from 'origin'.

### Run the CLI:

``` bash
cargo run > circle.geojson
```

    ##    Compiling zonebuilder v0.1.0 (/home/robin/github-orgs/zonebuilders/zonebuilder-rust)
    ##     Finished dev [unoptimized + debuginfo] target(s) in 0.65s
    ##      Running `target/debug/zonebuilder`

Take a look at the output:

``` bash
head -n 20 circle.geojson
```

    ## {
    ##   "features": [
    ##     {
    ##       "geometry": {
    ##         "coordinates": [
    ##           [
    ##             [
    ##               1.0,
    ##               0.0
    ##             ],
    ##             [
    ##               0.998629,
    ##               0.052335
    ##             ],
    ##             [
    ##               0.994521,
    ##               0.104528
    ##             ],
    ##             [
    ##               0.987688,

### Then read in the GeoJSON file with another tool, e.g. R (this step runs from an R console that has the `sf` library installed):

``` r
circle = sf::read_sf("circle.geojson")
plot(circle)
```

![](README_files/figure-gfm/circle-1.png)<!-- -->

``` r
# interactive version:
# mapview::mapview(circle)
file.remove("circle.geojson")
```

    ## [1] TRUE

<!-- ## Tidy up -->
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
