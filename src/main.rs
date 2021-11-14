use ytd_rs::{YoutubeDL, ResultType, Arg};
use std::path::PathBuf;

fn main() {
    // Set some testing details
    let test_url = "https://www.youtube.com/watch?v=TIgcVT4SfGw";
    let output_path = PathBuf::from("./");

    // Define the args
    let arg_list = vec![Arg::new_with_arg("-f", "bestaudio"), Arg::new_with_arg("--postprocessor-args", "-metadata artist='Mori Calliope'")];

    // Do the actual download
    let downloader = YoutubeDL::new(&output_path, arg_list, test_url).unwrap();
    let dl_res = downloader.download();

    // Make sure it's not an error
    match dl_res.result_type() {
        ResultType::SUCCESS => println!("Downloaded :)"),
        _ => println!("Failed to download: {:?}", dl_res.output())
    };
}
