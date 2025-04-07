//Napisz funkcje o naglowku: fn liczba_wystapien(napis: ...., znak: ....) -> .... ktora zliczy i
//zwroci ile jest danych znakow w danym napisie


fn liczba_wystapien(napis: &str, znak: char) -> usize {
    let mut liczba_ile_wystapien = 0;
    for i in napis.chars(){
       if i == znak{
        liczba_ile_wystapien+=1; 
       }
    }
    liczba_ile_wystapien
}


fn main (){
    let napis = "Ala ma kota";
    let znak = 'a';
    println!("Liczba wystapien litery {} w napisie {:?}  jest rowna: {}", znak, napis, liczba_wystapien(napis,znak));

}
