//1. Napisz program, ktory wyswietla informacje o przestepnosci danego roku
fn is_it_leap_year(year: &i32) -> bool{
        if year % 4 == 0 {
                if year % 100 == 0 {
                         if year % 400 == 0{
                                return  true;
                        } else {
                                return false;
                        }
                }
                else {
                        return true;
                }
        }
   false
}

fn main() {
    let year = 2024;
    println!("Is it year is leap year? : {}", is_it_leap_year(&year));
}


~       
