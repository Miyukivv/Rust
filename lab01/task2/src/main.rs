//2. Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku. 
fn how_many_days(number_of_month: i32, number_of_year: &i32) -> i32{
   if number_of_month == 2{
      if is_it_leap_year(&number_of_year) {
          29
      } else {
          28
      }
   } else  if number_of_month % 2 == 0 {
       30
   } else {
      31
   }

}

fn main() {
   let year: i32 = 2024;
   let month: i32 = 3;
   println!("Miesiac {} ma {} dni w {} roku", month, how_many_days(month,&year),year);
}
