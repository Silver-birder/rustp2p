use image::GenericImageView;
use walkdir::WalkDir;
use regex::Regex;

pub fn rename() {
    let wark = WalkDir::new("./lake/a").sort_by_file_name();
    let mut i = 1;
    for a in wark {
        let dir = a.unwrap();
        if dir.file_type().is_dir() {
            continue;
        }
        let result = image::open(dir.path());
        if result.is_err() {
            continue;
        }
        let mut img = result.unwrap();
        let s = format!("{:08}", i);
        let dir_path = dir.path().to_str().unwrap().replace(dir.file_name().to_str().unwrap(), "");
        let dist_path = dir_path + &s + ".jpeg";
        if img.width() > img.height() {
            let mut left_img = img.crop(0, 0, img.width() / 2, img.height());
            let mut right_img = img.crop(img.width() / 2, 0, img.width() / 2, img.height());
            while left_img.width() * left_img.height() > 500 * 1000 {
                println!("{:?}", left_img.dimensions());
                let w = left_img.width() as f64;
                let h = left_img.height() as f64;
                left_img = left_img.resize(
                    ((w * 0.9).round() as i64).try_into().unwrap(),
                    ((h * 0.9).round() as i64).try_into().unwrap(),
                    image::imageops::FilterType::CatmullRom,
                );
            }
            left_img.save(dist_path).unwrap();
            i = i + 1;

            while right_img.width() * right_img.height() > 500 * 1000 {
                println!("{:?}", right_img.dimensions());
                let w = right_img.width() as f64;
                let h = right_img.height() as f64;
                right_img = right_img.resize(
                    ((w * 0.9).round() as i64).try_into().unwrap(),
                    ((h * 0.9).round() as i64).try_into().unwrap(),
                    image::imageops::FilterType::CatmullRom,
                );
            }
            let s = format!("{:08}", i);
            let dir_path = dir.path().to_str().unwrap().replace(dir.file_name().to_str().unwrap(), "");
            let dist_path = dir_path + &s + ".jpeg";
            right_img.save(dist_path).unwrap();
            i = i + 1;
        } else {
            while img.width() * img.height() > 500 * 1000 {
                println!("{:?}", img.dimensions());
                let w = img.width() as f64;
                let h = img.height() as f64;
                img = img.resize(
                    ((w * 0.9).round() as i64).try_into().unwrap(),
                    ((h * 0.9).round() as i64).try_into().unwrap(),
                    image::imageops::FilterType::CatmullRom,
                );
            }
            i = i + 1;
            img.save(dist_path).unwrap();
        }
    }
}

pub fn reimage() {
    println!("reimage");
}

pub fn reduce() {
    println!("reduce");
}
