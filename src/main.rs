mod args;
use args::Args;
use image::{
    imageops::filterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};
use std::{fs::File, io::BufReader};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();

    let (image1, image_format1) = find_img(args.img1);
    let (image2, image_format2) = find_img(args.img2);

    if image_format1 != image_format2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    resize_img_size(image1, image2);
    Ok(())
}

// open img with the given path, get the format and decode it
// return the decoded img and the format
fn find_img(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();

    (image, image_format)
}

fn get_smallest_img(res1: (u32, u32), res2: (u32, u32)) -> (u32, u32) {
    let pix1 = res1.0 * res1.1;
    let pix2 = res2.0 * res2.1;

    return if pix1 < pix2 { res1 } else { res2 };
}

fn resize_img_size(img1: DynamicImage, img2: DynamicImage) //-> (DynamicImage, DynamicImage)
{
    let (width, height) = get_smallest_img(img1.dimensions(), img2.dimensions());
    //debug
    println!("Width: {}\nheight: {}\n", width, height);

    if img2.dimensions() == (width, height) {
        (img1.resize_exact(width, height, Triangle), img2)
    } else {
        (img1, img2.resize_exact(width, height, Triangle))
    }
}
