// fn main(){
//     let x = 3;

//     let rem = x%3;

//     if rem == 0 {
//         println!("Divisible");

//     }else if {
//         println!("remainder = {}", rem);
//     }
// }
fn borrow(w:& String){
   println!("{}",w)
}




fn main () {
    let word = "UTRGV".to_string();
    borrow(&word);

}