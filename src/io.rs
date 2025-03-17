use std::io;
use std::io::stdin;

pub fn run (){
    let mut age= String::new() ;
     io::stdin().read_line(&mut age ).expect("failed to readline");
    let age:i32 = age.trim().parse().expect("not a number");
}