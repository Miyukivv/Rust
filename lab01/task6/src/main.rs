//6 Napisz program, ktory oblicza silnie dla danej liczby
fn factorial(number: i64) -> i64{
    let mut result=1;
    let mut n=number;
    while n >1 {
        result*=n;
        n-=1;

    }
    result
}

fn main (){
    let number = 5;

    println!("factorial of the number {} is {}",number,factorial(number));

}
