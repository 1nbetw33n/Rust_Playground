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

struct List(Vec<i32>);
impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result{
        let vec = &self.0;
        write!(f, "[")?;
        for(count, v) in vec.iter().enumerate(){
            if count != 0{ write!(f, ", ")?;}
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}


pub fn testcase_list_main(){
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}