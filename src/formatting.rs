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

use std::fmt::{self, Formatter, Display};


struct City{
    name: &'static str,
    latitude: f32,
    longitude: f32,
}

impl Display for City{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let latitude_c = if self.latitude >= 0.0 {'N'} else {'S'};
        let longitude_c = if self.longitude >= 0.0 {'E'} else {'W'};
        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.latitude.abs(),
            latitude_c,
            self.longitude.abs(),
            longitude_c
        )
    }
}

#[derive(Debug)]
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt:: Result{
        write!(f, "RGB  ({}, {}, {})  0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
    }

}

pub fn formatting_main(){
    for city in[
        City {name: "Dublin", latitude: 53.347778, longitude: -6.259722},
        City {name: "Oslo", latitude: 59.95, longitude: 10.75},
        City {name: "Vancouver", latitude: 49.25, longitude: -123.1},
    ].iter(){
        println!("{}", *city);
    }
    for color in [
        Color{red: 128, green: 255, blue: 90},
        Color{red: 0, green: 3, blue: 254},
        Color{red: 0, green: 0, blue: 0},
    ].iter(){
        println!("{:?}", *color);
    }
    for color in [
        Color{red: 128, green: 255, blue: 90},
        Color{red: 0, green: 3, blue: 254},
        Color{red: 0, green: 0, blue: 0},
    ].iter(){
        println!("{}", *color);
    }
}