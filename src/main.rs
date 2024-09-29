use std::fmt::format;
use std::process::Command;
use colored::Colorize;
use num_integer::Roots;
use image::ImageReader;
use dotenv::dotenv;

fn main() {
    let year = 2015;
    let git_repo = "git-repo";
    let dot = dotenv().expect("TODO: panic message");

    let github_token = dotenv::var("GITHUB_TOKEN").expect("NO GITHUB TOKEN IN ENV");

    println!("{}", github_token);



    // println!("{}", cmd);

    // cmd are the git commits



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

fn convert_image_to_tiles(year: i32, path: &str) -> Vec<u8> {
    let image_name = path;
    let img = ImageReader::open(image_name).unwrap().decode().unwrap().to_luma8();


    if (img.height() != 7 || img.width() >= 50) {
        panic!("Image dimensions are wrong or too large: w ({}), h ({}) but should be h (7), w (< 50)", img.width(), img.height())
    }

    let pixel_data: Vec<u8> = img.as_raw().iter().map(|x| ((*x as f64 / 255f64) * 4f64) as u8).collect();


    //region log pixel art to
    let opacities = [" ", ".", "o", "O", "X"];

    //endregion


    let is_leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days_of_year = if is_leap_year { 366 } else { 365 };

    let date: (i32, i32, i32) = day_to_date(year, 30);
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
        // let col = i - row;

        if i >= start_index && i <= end_index {
            let sub_i = i - start_index;

            let image_row = sub_i % height;
            let image_col = (sub_i - image_row) / height;

            let image_i: usize = (image_row * img.width() as i32 + image_col) as usize;

            green_tiles_data.push(pixel_data[image_i]);
        } else {
            green_tiles_data.push(0);
        }
    }



    let mut cmd = "".to_string();

    for (i, color) in green_tiles_data.iter().enumerate() {
        for c in 0..*color {
            let date: (i32, i32, i32) = day_to_date(year, i as i32);
            cmd.push_str(format!("git commit -m '{:0>4}-{:0>2}-{:0>2}__{}' --date {:0>4}-{:0>2}-{:0>2}\n", year, date.1, date.2, i, year, date.1, date.2).as_str());
        }
    };

    green_tiles_data
}