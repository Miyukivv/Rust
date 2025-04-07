/*
Zdefiniuj funkcję o nagłówku

fn szyfruj(napis: ..., klucz: ...) -> ...

która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów. Przykłady:

szyfruj("Aladyn", 2) == "lAdany"
szyfruj("Aladyn", 3) == "alAnyd"
szyfruj("Aladyn", 4) == "dalAny"
szyfruj("Aladyn", 5) == "ydalAn"
szyfruj("koza", 3) == "zoka"
szyfruj("kaszanka", 3) == "saknazak"
szyfruj("kot Mruczek", 9) == "zcurM tokke"
szyfruj("kot Mruczek", 1) == "kot Mruczek"
szyfruj("kot Mruczek", 2) == "ok trMcuezk"
*/

fn szyfruj(napis: &str, klucz: usize) -> Vec<char>{
    let mut result: Vec<char> = Vec::new();

    for c in (0..napis.len()).step_by(klucz){
        let mut end = c+klucz;
        if end >  napis.len() {
            end=napis.len();
        } 

        let fragment = &napis[c..end];
        let rev: String = fragment.chars().rev().collect();
        result.extend(rev.chars());
    }
    result

}
fn main (){

    println!("{:?}", szyfruj("Aladyn", 2)); 
    println!("{:?}", szyfruj("Aladyn", 3)); 
    println!("{:?}", szyfruj("Aladyn", 4)); 
    println!("{:?}", szyfruj("Aladyn", 5)); 
    println!("{:?}", szyfruj("koza",3)); 
    println!("{:?}", szyfruj("kaszanka",3)); 
    println!("{:?}", szyfruj("kot Mruczek",9)); 
    println!("{:?}", szyfruj("kot Mruczek",1)); 
    println!("{:?}", szyfruj("kot Mruczek",2)); 
}
