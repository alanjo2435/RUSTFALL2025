
fn append_region(word: &mut String) {
    word.push_str("RGV");
}

fn main(){
    let mut x = "UT".to_string();
    append_region(&mut x);


    println!("{}", x);
}