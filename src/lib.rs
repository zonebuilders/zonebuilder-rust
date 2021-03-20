use geo::{Point, Polygon};

pub fn clockboard(centerpoint: Point<f64>, num_segments: usize, radii: Vec<f64>, boundary: Option<Polygon<f64>>) -> Vec<Polygon<f64>> {
   Vec::new() 
}  

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("Hello world! {}", "test");
        assert!(false)
    }
}
