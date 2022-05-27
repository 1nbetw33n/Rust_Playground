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
 */#![allow(unused_variables)]

//
//created by 0x1nbetw33n on 27/05/2022
//

//noinspection ALL
pub fn formatted_print_main(){
    println!("Hello, world!");
    println!("{} days", 31);
    println!("{0}, this is {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");
    println!("{} of {:b} humans know binary, the other half doesnt", 1, 2);
    println!("{number:>width$}", number =1, width =6);
    println!("{number:0>width$}", number = 1, width = 6);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure<'a>{
        name: &'a str,
        size: u8,
    }
    impl std::fmt::Display for Structure<'_> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(fmt, "Name: {} \nSize: {}", self.name, self.size)
        }
    }

    let custom_structure = Structure{
        name: "Purp",
        size: 15,
    };

    println!("{}", custom_structure);
    #[allow(dead_code)]
    struct Structure2(i32);

    //println!("This struct `{}` won't print...", Structure2(3));
    let number: f64 = 1.;
    let width: usize = 6;
    println!("{number:>width$}");

    //noinspection RsApproxConstant
    const PI: f64 = 3.141592;
    const PI2: f64 = std::f64::consts::PI;

    println!("PI: {:.3} | PI2: {:.3}", PI, PI2);
}