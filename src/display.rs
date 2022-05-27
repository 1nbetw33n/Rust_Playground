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

use std::fmt;
use std::fmt::Formatter;

struct Structure(i32);

impl fmt::Display for Structure{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}) | ({})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex{
    re: f64,
    im: f64,
}

impl fmt::Display for Complex{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "{} + {}i", self.re, self.im)
    }
}

pub fn display_main(){
    let minmax = MinMax(7, 16);
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    let point2d = Point2D{x: 0.3, y: 7.4};
    let complex = Complex{re: 3.3, im: 7.2};
    println!("{}", minmax);
    println!("{:?}", minmax);
    println!("small_range: {smallRange} | big_range: {bigRange}",
                    smallRange = small_range,
                    bigRange = big_range);
    println!("{}", point2d);
    println!("{:?}", point2d);
    println!("{}", complex);
    println!("{:?}", complex);
}