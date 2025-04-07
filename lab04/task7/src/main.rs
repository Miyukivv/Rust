/*
Napisz funkcję o nagłówku

fn dodaj_pisemnie(a: ..., b: ...) -> ...

która doda dwie (zakładamy, że poprawne) liczby całkowite podane w argumentach jako napisy w zapisie dziesiętnym — i zwróci wynik również jako napis. Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże. Przykłady:

dodaj_pismnie("1", "3") == "4"
dodaj_pismnie("1", "3") == "4"
dodaj_pismnie("8", "3") == "11"
dodaj_pismnie("10", "23") == "33"
dodaj_pismnie("1", "0") == "1"
dodaj_pismnie("11", "00") == "11"
dodaj_pismnie("131", "9900") == "10031"
dodaj_pismnie("998", "7") == "1005"
dodaj_pismnie("24872947", "294729478") == "319602425"
dodaj_pismnie("5924729874298749827418582", "6782893629472094209740298") == "12707623503770844037158880"
*/

fn dodaj_pisemnie(a: &str, b: &str) -> String {

    let mut wynik = String::new();
    let mut przeniesienie = 0;

    let cyfry_a: Vec<u8> = a.bytes().rev().collect();
    let cyfry_b: Vec<u8> = b.bytes().rev().collect();

    let max_dlugosc = std::cmp::max(cyfry_a.len(), cyfry_b.len());

    for i in 0..max_dlugosc {
        let cyfra_a;

        if i < cyfry_a.len() {
            cyfra_a = (cyfry_a[i] as char).to_digit(10).unwrap();

        } else {
            cyfra_a = 0;
        }

    let cyfra_b;
    
    if i < cyfry_b.len() {
        cyfra_b = (cyfry_b[i] as char).to_digit(10).unwrap();
    } else {
        cyfra_b = 0;
    }

    let suma = cyfra_a + cyfra_b + przeniesienie;

    wynik.push(std::char::from_digit(suma % 10, 10).unwrap());
    przeniesienie = suma / 10;
    }



    if przeniesienie > 0 {
        wynik.push(std::char::from_digit(przeniesienie, 10).unwrap());
    }

    wynik.chars().rev().collect()
}

fn main() {
    println!("{}", dodaj_pisemnie("1", "3"));
    println!("{}", dodaj_pisemnie("8", "3")); 
    println!("{}", dodaj_pisemnie("10", "23")); 
    println!("{}", dodaj_pisemnie("1", "0")); 
    println!("{}", dodaj_pisemnie("11", "00"));
    println!("{}", dodaj_pisemnie("131", "9900"));
    println!("{}", dodaj_pisemnie("998", "7"));
    println!("{}", dodaj_pisemnie("24872947", "294729478"));
    println!("{}", dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298")); 

}
