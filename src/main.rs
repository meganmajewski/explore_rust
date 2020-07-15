use std::mem;
fn data_types(){
    let a:u8 = 123; //unsigned 8 bit memory location
    println!("a = {}", a);

    //try to give a a new value
   //NOT:  a = 456; variable bindings are immutable

   let mut b:i8 = 0; //mutable
   println!("b = {}",b);
   b=42;
   println!("b = {}",b);
   
   //rust can assume type
   let c = 123456789; //would be i32;
   println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

   // other types
   let d:char = 'x'; //default behavior is 4 bits
   println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));
   
   let e = 2.5;  //double is 8 bytes or 64 bits type f64;
   println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));
   
   let g = false; //1 byte but prints true / false;
   println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
   let  f = 4>0; //true

}
fn operators(){
    let mut a = 2+3*4; 
    println!("{}",a);
    a = a+1; //does not support ++ 

    let a_cubed = i32::pow(a,3); //use power from i32 type to calculate power of something
    println!("{} cubed is {}", a, a_cubed);
    
    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("4.0 is less than pie {}", pi_less_4);
    
    let x = 5;
    let x_is_five = x == 5;
    println!("{} is equal to {}", x, x_is_five);
    

}
fn main() {
    data_types();
    operators();
}