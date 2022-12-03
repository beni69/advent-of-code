use std::{env, error::Error};

pub fn upload(s: &str) -> Result<String, Box<dyn Error>> {
    let token = env::var("PASTEBIN_KEY")?;

    eprintln!(
        "sending request: {} bytes, {} lines",
        s.len(),
        s.lines().count()
    );
    let res = ureq::post("https://pastebin.com/api/api_post.php")
        .send_form(&[
            ("api_dev_key", &token),
            ("api_option", "paste"),
            ("api_paste_code", s),
        ])?
        .into_string()?;
    Ok(res.replacen("pastebin.com/", "pastebin.com/raw/", 1))
}

pub fn download(s: &str) -> Result<String, Box<dyn Error>> {
    eprintln!("sending request: {}", s);
    let res = ureq::get(s).call()?;
    eprintln!("status: {}", res.status());
    Ok(res.into_string()?)
}
