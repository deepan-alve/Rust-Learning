use crate::printconstant;

pub fn run ()
{
    let person : (&str,&str,i32) = ("deepan","deep",32);
    println!("{} is {} in troble so he shoud run {} times" , person.0, person.1, person.2);
}