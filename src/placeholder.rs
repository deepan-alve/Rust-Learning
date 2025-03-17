pub fn run () {
    println!("{} hi i am {}", "hey", "deepan");
    // positional placehodler argument
    println!("{0} hey {1} , {0}", "oiii", "deepan");
    // named arguemnts 
    println!("hi {name} , how are you {talker} , he aaked ?" , name = "Deepan", talker = "oiii");
    // traits 
    println!("Bin {0:b} , hex  {0:x} , octal {0:o} ",10);
    // debugger 
    println!("{:?}",(12,10,true));
    // Math Functions 
    print!("10 +10 = {}", 10+10)
}
