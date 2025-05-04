
fn main() {

    let src = std::fs::read_to_string("cookies.txt").unwrap();
    let json: serde_json::Value = serde_json::from_str(&src).expect("JSON was not well-formatted");

    // use std::io::Write;
    // let mut writer = std::io::BufWriter::new("cookies2.txt");
    let mut out=String::new();

    for x in json.as_array().unwrap() {
        let x=x.as_object().unwrap();
        let domain=x.get("domain").and_then(|x|x.as_str()).unwrap_or("");

        if domain.contains("woolworths") {
            // println!("{:?}",x);
            // serde_json::to_writer(&mut writer, &x).unwrap();
            let y=serde_json::to_string(&x).unwrap();
            println!("{y}");

            if !out.is_empty() {
                out+=",";
            }
            out+=&y;
            // serde_json::to_writer(&mut writer, "".to_string()).unwrap();
        }

        // writer.flush().unwrap();

        // println!("{:?}",x.get("domain"));
    }

  //
  let path="cookies2.txt";
  let Ok(mut output) = std::fs::File::create(path) else {
    eprintln!("Cannot create file: {path:?}");
    return ;
    };

    use std::io::Write;

    if write!(output, "[{}]",out).is_err() {
        eprintln!("Cannot write file: {path:?}");
        return ;
    }


}