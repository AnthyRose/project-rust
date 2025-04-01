use std::fs::File;

fn CountFiles(images: &str) {
    let mut count = 0;
    for image in images.lines() {
        if image.ends_with(".png") || image.ends_with(".jpg") {
            count += 1;
        }
    }
    println!("Total number of image files: {}", count);
}

fn openImages(images: &str) {
    let arr: Box<[i32]> = Box::new([0; 10]);
    for image in images.lines() {
        if image.ends_with(".png") || image.ends_with(".jpg") {
            let mut arr = File::open(image).expect("Failed to open file");
        }
    }
}

fn main() {
    let images = "./images/*";
    let countingFiles = CountFiles(images);
    let openFiles = openImages(images);
}
