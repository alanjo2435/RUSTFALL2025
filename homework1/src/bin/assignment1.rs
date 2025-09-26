// Assignment 1: Temperature Converter
fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    let mut temp_f: f64 = 32.0;

    println!("{}°F = {:.2}°C", temp_f, f_to_c(temp_f));
    
    for _ in 1..=5 {
        temp_f += 1.0;
        println!("{}°F = {:.2}°C", temp_f, f_to_c(temp_f));
    }
}
