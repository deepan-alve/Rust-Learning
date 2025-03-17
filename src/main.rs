mod print;
mod printconstant;
mod placeholder;
mod variable;
mod types;
mod controlflow;
mod io;

fn main() {
    print::run();
    printconstant::run();
    placeholder::run();
    variable::run();
    types::run();
    controlflow::run();
}
