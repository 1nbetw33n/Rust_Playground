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

pub fn primitives_main(){
    let logical: bool = true;

    let a_float: f64 = 1.; //REGULAR ANNOTATION
    let an_integer = 5i32; //SUFFIX ANNOTATION

    let default_float = 3.; //f64
    let default_integer = 7; //i32
    let mut inferred_type  =12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    //mutable = true; //ERROR: CANT CHANGE VARIABLE TYPE

    let mutable = true; //SHADOWING
}