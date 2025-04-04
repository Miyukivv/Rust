//8 Napisz program, ktory oblicza sume cyfry danej liczby calkowitej

fn main (){
    let mut number = 123;
    let mut digit;
    let mut sum = 0;


    while number>0{
        digit = number % 10;
        sum+=digit;
        number /= 10;
    }
    println!("Sum of digits is {}", sum);
}
