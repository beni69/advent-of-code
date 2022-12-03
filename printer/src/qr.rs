use fast_qr::{
    convert::{image::ImageBuilder, Builder, Shape},
    QRBuilder, ECL,
};
use image::{imageops::FilterType, DynamicImage, Luma};
use std::{error::Error, path::Path};

pub fn decode(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let img = image::io::Reader::open(path)?.decode()?;
    let img = process(&img)?;
    let l = img.to_luma8();
    let mut i = rqrr::PreparedImage::prepare(l);
    let grids = i.detect_grids();
    assert!(!grids.is_empty(), "no qr code found");
    let mut res = Vec::new();
    for grid in grids {
        let (_meta, content) = grid.decode().unwrap();
        dbg!(_meta);
        res.push(content);
    }
    Ok(res)
}

pub fn encode(s: String, path: &str) -> Result<(), Box<dyn Error>> {
    let qr = QRBuilder::new(s).ecl(ECL::H).build().unwrap();

    ImageBuilder::default()
        .shape(Shape::Square)
        .fit_width(600)
        .to_file(&qr, path)
        .unwrap();

    Ok(())
}

fn process(img: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>> {
    eprintln!("processing image");
    let mut luma = img.to_luma8();

    for i in 0..luma.width() {
        for j in 0..luma.height() {
            let p = luma.get_pixel(i, j).0[0];

            let around = [
                (i, j + 1),
                (i, j - 1),
                (i + 1, j),
                (i - 1, j),
                (i + 1, j + 1),
                (i + 1, j - 1),
                (i - 1, j - 1),
                (i - 1, j + 1),
            ]
            .into_iter()
            .filter_map(|(x, y)| {
                if x < luma.width() && y < luma.height() {
                    Some(luma.get_pixel(x, y)[0] == p)
                } else {
                    None
                }
            })
            .all(|x| x);

            if around {
            } else if p < 180 {
                luma.put_pixel(i, j, Luma([0]));
            } else {
                luma.put_pixel(i, j, Luma([255]));
            }
        }
    }

    // luma.save("luma.png")?;
    let img = DynamicImage::ImageLuma8(luma).resize(
        img.width() / 10,
        img.height() / 10,
        FilterType::CatmullRom,
    );
    // img.save("resized.png")?;

    eprintln!("processing done");
    Ok(img)
}
