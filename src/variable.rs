use std::ffi::CString;

pub fn run() {
    let name = "Brad";
    println!("My name is {}", name);
    let mut age = 30;
    println!("My name is {} and I am {}", name, age);
    age = 40;
    println!("My name is {} and I am {}", name, age);
    //const
    const SEX:&str = "male";
    println!("My name is {} and i am {} and i am a {}",name,age,SEX);

    // multiple initializations 
    let (na,ag,se)=("Brad",32,"male");
    println!("My name is {} and i am {} and i am a {}",na,ag,se);
    
}