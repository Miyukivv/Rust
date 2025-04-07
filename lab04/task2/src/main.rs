/*Napisz funkcję o nagłówku

fn rzymskie(napis: ...) -> ...

która dla napisu reprezentującego liczbę w zapisie rzymskim (zakładamy jego poprawność) zwraca liczbę reprezentowaną przez ów napis. Przykłady:

rzymskie("III") == 3
rzymskie("IX") == 9
rzymskie("XIX") == 19
rzymskie("MCMX") == 1910
*/


fn rzymskie(napis: &str) -> usize{
   let mut sum = 0;
   let mut previous = 0;

   for c in napis.chars().rev(){
       let current = match c {
           'I' => 1,
           'V' => 5,
           'X' => 10,
           'L' => 50,
           'C' => 100,
           'D' => 500,
           'M' => 1000,
           _ => 0
       };
       
       if current < previous {
            sum -= current;
       } else {
            sum += current;
       }
       previous=current;
   }
   sum
}

fn main (){
    println!("{}", rzymskie("III"));
    println!("{}", rzymskie("IX"));
    println!("{}", rzymskie("XIX"));
    println!("{}", rzymskie("MCMX"));
}
