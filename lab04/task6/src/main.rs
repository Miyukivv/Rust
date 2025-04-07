
fn na_rzymskie(mut liczba: usize) -> String {
    let mut wynik: String = String::new();

    while liczba >= 1000 {
        wynik.push('M');
        liczba-=1000;
    } 
    
    if liczba >= 900 {
        wynik.push_str("CM");
        liczba -=900;
    } 
    if liczba >= 500 {
        wynik.push('D');
        liczba-=500;
    }
    if liczba >= 400 {
        wynik.push_str("CD");
        liczba-=400;
    }
    if liczba >= 100 {
        wynik.push('C');
        liczba-=100;
    }

    if liczba >= 90 {
        wynik.push_str("XC");
        liczba -=90;
    }

    if liczba >= 50 {
        wynik.push('L');
        liczba-=50;
    }

    if liczba >= 40 {
        wynik.push_str("XL");
        liczba -=40;
    }

    if liczba >= 10 {
        wynik.push('X');
        liczba-=10;
    }

    if liczba >= 9 {
        wynik.push_str("IX");
        liczba-=9;
    }

    if liczba >= 5{
        wynik.push('V');
        liczba -=5;
    }
    if liczba >= 4 {
        wynik.push_str("IV");
        liczba -=4;
    }

    if liczba >= 1{
        wynik.push('I');
        liczba -=1;
    }
    wynik
}

fn main (){
    println!("{}", na_rzymskie(3));
    println!("{}", na_rzymskie(9));
    println!("{}", na_rzymskie(19)); 
    println!("{}", na_rzymskie(1910)); 
}
