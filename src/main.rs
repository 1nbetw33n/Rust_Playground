/*
 *  copyright (c) 2021 1nbetw33n Labs.
 *  Planet Earth, Milky Way, Virgo Supercluster.
 *  All rights reserved.
 *
 * This software is the confidential and proprietary information of
 * 1nbetw33n Labs. ("Confidential Information"). You shall not disclose
 * such Confidential Information and shall use it only in accordance
 * with the terms of the license agreement you entered into with
 * 1nbetw33n Labs.
 *
 */

//
//created by 0x1nbetw33n on 27/05/2022
//

use crate::display::display_main;
use crate::formatted_print::formatted_print_main;
use crate::formatted_print_debug::formatted_print_debug_main;
use crate::formatting::formatting_main;
use crate::literals_and_operators::literals_and_operators_main;
use crate::primitives::primitives_main;
use crate::testcase_list::testcase_list_main;
use crate::tuples::tuples_main;

mod formatted_print;
mod formatted_print_debug;
mod display;
mod testcase_list;
mod formatting;
mod primitives;
mod literals_and_operators;
mod tuples;


fn main() {
    //formatted_print_main();
    //formatted_print_debug_main();
    //display_main();
    //testcase_list_main();
    //formatting_main();
    //primitives_main();
    //literals_and_operators_main();
    tuples_main();
}
