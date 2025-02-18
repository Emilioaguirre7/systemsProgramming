fn fahrenheit_to_celsius(f: f64) -> f64{
    return (f - 32.0) * 5.0/9.0
 }
 
 fn celsius_to_fahrenheit(c: f64) -> f64{
     return (c * 9.0 / 5.0) + 32.0
 }
 
 fn main () {
     let mut freezing_point_f = 32.0;
     let mut freezing_point_c = 0.0;
    println!("Farennheit to Celsius");

     for i in 1..=5{
        let c = fahrenheit_to_celsius(freezing_point_f);
        println!("{}",c);
        freezing_point_f += 1.0;
     }
     println!("Celsius to Farennheit");

     for i in 1..=5{
        let f = celsius_to_fahrenheit(freezing_point_c);
        println!("{}",f);
        freezing_point_c += 1.0;
     }
 }