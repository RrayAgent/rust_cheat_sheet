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
mod basics;
//mod + name of file = local imports

fn main() {
    //basics::print();
    //basics::noticing_print_diff();
    //basics::vars();
    basics::loops_funcs_and_ownership()
}
