fn main() {
    println!("Hello, world!");

    // Compound types 
    let tup = ("Let's get party", 20_232);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let x = error_codes[3];
    
    let byte = [0; 8];

}
