extern crate tyr;

fn main() {

    if let Err(err) = tyr::write_csv() {
        println!("Error: {}", err);
    }
}
