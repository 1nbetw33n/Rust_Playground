/*
 *  copyright (c) 2022 1nbetw33n Labs.
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

struct UnPrintable(i32);
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Human<'a>{
    name: &'a str,
    age: u8
}

pub fn formatted_print_debug_main(){
    //PRINTING WITH {:?}
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name. ",
                    "Slater",
                    "Christian",
                    actor = "actor's");
    //PRINTING STRUCTURES WITH DERIVED PRINT
    println!("Now {:?} will print!:)", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Purple";
    let age = 33;
    let purple = Human{name, age};
    //PRETTY PRINTING
    println!("{:#?}", purple)
}