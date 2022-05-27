use crate::display::display_main;
use crate::formatted_print::formatted_print_main;
use crate::formatted_print_debug::formatted_print_debug_main;

mod formatted_print;
mod formatted_print_debug;
mod display;


fn main() {
    //formatted_print_main();
    //formatted_print_debug_main();
    display_main();
}
