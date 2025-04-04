//4. Zamien Fahrenheity na stopnie Celsjusza

fn to_celsius(degree_in_fahrenheit: f64) -> f64 {
    (degree_in_fahrenheit-32.0)*5.0/9.0
}

fn main (){
    let  degree_in_fahrenheit = 41.0;
    println!("Degree in celsius: {}", to_celsius(degree_in_fahrenheit));
}
