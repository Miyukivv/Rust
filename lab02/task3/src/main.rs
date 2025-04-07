//3 Wyświetl tabelę widzialnych znaków ASCII wraz kodami (od 33 do 126).

fn show(){
    for i in 33..=126{
        let znak = i as u8 as char;
        println!("Kod: {:3} | Znak: {}", i, znak);
    }
}

fn main(){
    show();
}
