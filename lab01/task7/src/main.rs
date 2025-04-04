//7 Napisz program, ktory wyswietla cyfry danej liczby calkowitej (od konca)

fn main (){
    let mut number = 123;

    let mut digit;

    while number > 0 {
        digit = number % 10;
        println!("{}", digit);
        number /=10;
    }
}
