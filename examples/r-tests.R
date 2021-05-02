system("cargo run --example clockboard-example > zones_uk.geojson")
zones = sf::read_sf("zones_uk.geojson")
plot(zones)
# interactive version with basemap
library(tmap)
tmap_mode("view")
qtm(zones)
