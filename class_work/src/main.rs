fn concat_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone();
    cloned.push_str("Garcia");
    cloned
}

fn sum(total: &mut i32, low: i32, high: i32) {
    for i in low..=high {
        *total += i;
    }
}

fn main() {
    let a = String::from("Alan ");
    let b = String::from("Garcia");
    println!("{}", concat_strings(&a, &b));

    let c = String::from("Alan ");
    println!("{}", clone_and_modify(&c));
    println!("{}", c);

    let mut total = 1;
    sum(&mut total, 1, 50);
    println!("{}", total);
}
