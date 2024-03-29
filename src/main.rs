use crate::advanced::{is_adult, Cop};

/*
to make an directory enter the cmd/power shell/terminal where you want the folder to appear
write : cargo new [file_type] [name for file]
file_types (or at least the most used):
    --bin = creates a project that will produse executable files (like this program)
        most useful ways of compiling:
            cargo run = creates a debug (not able to be ported to a different divice) 
                exicutable of your code and runs it
            cargo build = creates a debug exicutable of your code
            cargo build -r = creates a release (avialible for other divices to use) exicutable
            cargo run -r = creates a release exicutable and runs it
    --lib = creates a dynamic library (without making certain changes to the .toml file it will only comple for rust)
        most useful ways of compiling:
            cargo build = creates a debug library
            cargo build -r = creates a release library
*/
//Just a note if you want to time execution time on windows in power shell it is Measure-Command {command_name}
mod basics;
//mod + name of file = local imports
mod advanced;
fn main() {
    //basics::print();
    //basics::noticing_print_diff();
    //basics::vars();
    //basics::loops_funcs_and_ownership();
    let stru = advanced::Learner::new("David".to_string(),18, "python".to_string());
    stru.structs_classes();
    let d_stru = advanced::Learner::default();
    println!("{0} - {1}",&d_stru.name, &d_stru.p_code);
    println!("Is {0} adults", stru.name);
    if is_adult(&stru){
        println!("{0} is adults", stru.name);
    }else{
        println!("{0} is not adults", stru.name);
    }
    //cloning without re-assignment or memory leakage
    let d_copy = advanced::Learner::clone_from(&d_stru);
    println!("{0:#?}, {1:#?}", d_stru, d_copy);
//using libs implimented
    /*let k = advanced::using_libs();
    println!("{}", k.len());
    advanced::running_parralle(23);*/
    let y = vec![42,21,67];
    let mut max =y[0];
    for i in 0..y.len()-1{
        max = y[i].max(y[i+1]);
        println!("{max}")
    }
    println!("max = {max}")
}
