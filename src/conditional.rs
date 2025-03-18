pub fn run(){
    // IF ELSE 
    let mut age =18;
    if age<18{
        print!("ur are not allowed to drink ");
    }
    else if age>18 && age<21{ print!("ur below are not allowed to drink "); }
    else {
        print!("drink");
    }
    
    // Short Hand if else 
    let mut checker : bool = if age>21 {true} else {false};
}