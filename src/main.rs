use ytd_rs::{YoutubeDL, ResultType, Arg};
use std::path::PathBuf;

// A struct that will hold all the data about the song
#[derive(Debug)]
struct FileData
{
    title: Option<String>,
    file_name: Option<String>,
    extension: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    cover: Option<String>,
    output_path: Option<PathBuf>,
}

// An enum that will hold all the reasons anything could fail
#[derive(Debug)]
enum FailReason
{
    FieldNotProvided(String)
}

fn main() 
{
    // Create the file data that we'll work with throughout
    let mut song: FileData = FileData
    {
       title: None,
       file_name: None,
       extension: None,
       artist: None,
       album: None,
       cover: None,
       output_path: None
    };
    
    println!("Current song struct is\n{:?}", &song);

    // Pass the song to the argument parsing function
    if let Err(e) = parse_flags(&mut song)
    {
        println!("{:?}", e);
    }

    println!("song is now\n{:?}", &song);
}

fn parse_flags(file_data: &mut FileData) -> Result<(), FailReason>
{
    file_data.title = Some(String::from("Title will go here!"));
    Ok(())
}
