/* 5Napisz funkcję wizytowka, która otrzymuje w dwóch parametrach napisowych imię i nazwisko, a zwraca napis powstały z pierwszej litery imienia, kropki, spacji i nazwiska, przy czym w wyniku pierwsza litera imienia i nazwiska mają być duże, pozostałe małe. Na przykład, dla danych "jan" oraz "KOWALSKI" funkcja ma zwracać napis "J. Kowalski". Wskazówka: użyj metod to_lowercase oraz to_uppercase. 
*/


fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let mut result: String = String::new();
    let first_letter=imie.chars().next().unwrap().to_uppercase();
    result.push_str(&first_letter.to_string());
    result.push_str(". ");
    
    let mut chars = nazwisko.chars();
    if let Some(first_char) =chars.next(){
         result.push_str(&first_char.to_uppercase().to_string());
         result.push_str(&chars.as_str().to_lowercase());
     }
     result
}
fn main (){
    let imie = "Jan";
    let nazwisko = "Kowalski";

    println!("{:?}",wizytowka(imie,nazwisko));
}
