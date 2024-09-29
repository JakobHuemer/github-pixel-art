use std::fmt::format;
use colored::Colorize;
use num_integer::Roots;
use image::ImageReader;


fn main() {
    let year = 2015;

    let image_name = "assets/pixelart.jpg";
    let img = ImageReader::open(image_name).unwrap().decode().unwrap().to_luma8();

    // println!("{:?}", img.as_raw());

    if (img.height() != 7 || img.width() >= 50) {
        panic!("Image dimensions are wrong or too large: w ({}), h ({}) but should be h (7), w (< 50)", img.width(), img.height())
    }

    println!("------------------");
    // map from 255 to 0-4
    let pixel_data: Vec<u8> = img.as_raw().iter().map(|x| ((*x as f64 / 255f64) * 4f64) as u8).collect();

    println!("------------------");

    // println!("{:?}", pixel_data);

    //region log pixel art to
    let opacities = [" ", ".", "o", "O", "X"];

    for (i, p) in pixel_data.iter().enumerate() {
        if i % img.width() as usize == 0 {
            println!();
        }
        print!("{}", opacities[*p as usize]);
        print!("{}", opacities[*p as usize]);
    }

    println!();
    println!();

    //endregion


    let is_leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days_of_year = if is_leap_year { 366 } else { 365 };

    let date: (i32, i32, i32) = day_to_date(year, 30);
    println!("{}", is_leap_year);
    println!("{:?}", date);

    let mut green_tiles_data: Vec<u8> = Vec::new();

    let width: i32 = 52;
    let height: i32 = 7;
    let offset = first_weekday_of_year(year);
    let left_margin: i32 = (width - img.width() as i32) / 2;

    println!("offset: {}; lm: {}", offset, left_margin);
    println!("width: {}; height: {}", img.width(), img.height());


    let start_index = (left_margin * height) - offset;
    let end_index = start_index + img.width() as i32 * height - 1;

    for i in 0..offset {
        print!("    ");
    }

    for i in 0..days_of_year {
        let row = (i - offset + 1) % 7;
        let col = i - row;

      /*  if i == start_index {
            if (row == 0) {
                println!();
            }
            print!("{}", format!("{:0>3} ", i - start_index).on_red());
        } else if i == end_index {
            if (row == 0) {
                println!();
            }
            print!("{}", format!("{:0>3} ", i - start_index).on_cyan());
        } else */
        if i >= start_index && i <= end_index {
            if (row == 0) {
                println!();
            }
            let sub_i = i - start_index;

            let image_row = sub_i % height;
            let image_col = (sub_i - image_row) / height;

            let image_i: usize = (image_row * img.width() as i32 + image_col) as usize;

            let color = (pixel_data[image_i] as f32 / 4f32 * 255f32) as u8;
            print!("{}", format!("{}", "    ".on_truecolor(color, color, color)));
            green_tiles_data.push(color);
        } else {
            if (row == 0) {
                println!();
            }
            print!("{:0>3} ", i);

            green_tiles_data.push(0);
        }

        // if (col == 0) {print!("\n{}: ", row)}
    }


    println!();
    println!();

    println!("{:?}", green_tiles_data);

    
}

fn first_weekday_of_year(year: i32) -> i32 {
    let y = year - 1;
    ((year + y / 4 - y / 100 + y / 400 + 1) - 1) % 7
}

fn day_to_date(mut year: i32, mut day_number: i32) -> (i32, i32, i32) {
    let is_leap_year = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 1 } else { 0 };
    let month_days = [31, 28 + is_leap_year, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut total_month_days = 0;

    let mut month: i32 = -1;

    while day_number >= 0 {
        month = month + 1;
        if (month % 12 == 0 && month != 0) {
            year += 1;
            month = 0;
        }

        total_month_days += month_days[month as usize];
        day_number -= month_days[month as usize];
    }

    let day = month_days[month as usize] + day_number;

    (year, month + 1, day + 1)
}
