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
//created by 0x1nbetw33n on 28/05/2022
//

use std::fmt::{self, Formatter, Debug, Display};


fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]

struct Matrix (f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result{
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: (f32, f32, f32, f32)) -> (f32, f32, f32, f32){
    let (i11, i12, i21, i22) = matrix;
    (i11, i21, i12, i22)
}

pub fn tuples_main(){
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    println!("long_tuple 1. value: {}", long_tuple.0);
    println!("long_tuple 2. value: {}", long_tuple.1);
    println!("long_tuple 3. value: {}", long_tuple.2);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("{:?}", tuple_of_tuples);

    /*
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("{:?}", too_long_tuple);
*/

    let pair =(1i32, true);
    println!("{:?}", pair);
    println!("{:?}", reverse(pair));

    println!("one element tuple: {:?}", (4i64, ));
    println!("just a long: {:?}", (4i64));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
    println!("{}", transpose(matrix));

}