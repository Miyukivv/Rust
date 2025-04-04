//zadanie 3 - Napisz program sluzacy do konwersji wartosci temperatury podanej w stopniach
//Celsjusza na stopnie w skali Fahrenheita

fn to_fahrenheit(degree_in_celsius: f64) -> f64 {
    32.0+9.0/5.0*degree_in_celsius
}

fn main (){
    let mut degree_in_celsius = 5.0;

    println!("Degree in Fahrenheit: {}", to_fahrenheit(degree_in_celsius));
}
