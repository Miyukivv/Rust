//Napisz program, ktory dla danych dwoch poprawnych por jednej doby (w postaci calkowitych godzin,
//minut i sekund) wyswietla roznice czasow (takze w postaci analogicznej trojki, z minutami i
//sekundami w przedziale [0;59]}


fn main (){
    let g1 = 7;
    let m1 = 16;
    let s1 = 50;

    let g2 = 6;
    let m2 = 59;
    let s2 = 02;

    let to_second_1 = g1*3600 + m1 * 60 + s1;
    let to_second_2 = g2*3600 + m2 * 60 + s2;

    let mut total_result=to_second_1-to_second_2;

    if total_result < 0 {
        total_result=-total_result;
    }

    let gg_result = total_result/3600;
    let mm_result = (total_result % 3600) / 60;
    let ss_result = total_result % 60;

    println!("Roznica czasu wynosi: {} : {} : {}", gg_result,mm_result,ss_result);

}
