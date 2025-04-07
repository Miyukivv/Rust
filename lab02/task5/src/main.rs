//Napisz funkcje, ktora odpowiada na pytanie, czy jej argument jest liczba Armstronga

//liczba armstronga to liczba n  taka dla ktorej suma jej cyfr podniesionych do potegi liczby cyfr daje te
//sama liczbe  

fn armstrong(n: i64) -> bool{
    let mut number=n;
    let mut number_of_digits=0;

    while number > 0{
        let mut digit=n % 10;
        number = number/10;
        number_of_digits+=1;
    }
    
    number=n;
    let mut result = 0;
    
    while number > 0 {
        let mut digit = number % 10;
        result+=digit.pow(number_of_digits);
        number=number/10;
    }
     result == n
}

fn main (){
    let n = 153;
    if armstrong(n){
        println!("{} jest liczba Armstronga!", n);
    }
    else {
        println!("{} nie jest liczba Armstronga!", n);
    }
}
