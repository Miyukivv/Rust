/*Napisz funkcję pow_mod(x: u128, n: u128, p: u128) -> u128 która obliczy (xn)%p
w taki sposób, by działało to prawidłowo dla jak największych danych.
    Wskazówka 1: skorzystaj z własności reszty z dzielenia dla iloczynu (czy też inaczej: iloczynu modulo).
    Wskazówka 2: w celu ewentualnej optymalizacji czasowej użyj algorytmu szybkiego potęgowania.
*/

fn pow_mod(x: u128, n: u128, p: u128) -> u128{
    let mut result = 1;
    let mut base = x % p;

    let mut exp = n;

    while exp > 0 {
        if exp % 2 == 1{
            result = (result*base) %p;
        }

        base = (base * base) % p;
        exp /= 2;
    }
    result
}

fn main (){
    let x = 2;
    let n = 10;
    let p = 100;
    println!("(x^n) % p = {}", pow_mod(x,n,p));
}
