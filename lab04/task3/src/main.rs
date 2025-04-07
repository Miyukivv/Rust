//3 Napisz funkcje o naglowku: fn co_drugi_znak(napis: ...) -> ....

fn co_drugi_znak(napis: &str) -> Vec<String>{
    let mut vector: Vec<String> = Vec::new();
    for c in napis.chars().step_by(2){
        vector.push(c.to_string());
       // println!("{c:}");
    }
    vector
}


fn main (){
    let napis = "Ala ma kota"; 
    println!("Napis {:?} wyswietlony co drugi znak: {:?}",napis,co_drugi_znak(napis));
}
