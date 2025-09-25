fn assignment1(){
    //fahrenheit to celsius
    fn f_to_c(f: f64) -> f64{
        (f-32) * 5/9;
    }

    //celsius to fahrenheit
    fn c_to_f(c:f64) -> f64{
        (c * 9/5) + 32;
    }
}

fn main(){
    assignment1();
}
