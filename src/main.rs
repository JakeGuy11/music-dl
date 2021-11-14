extern crate ytd_rs;
extern crate regex;
use ytd_rs::{YoutubeDL, ResultType, Arg};
use std::path::PathBuf;
use std::io::Write;
use regex::Regex;

// A struct that will hold all the data about the song
#[derive(Debug)]
struct FileData
{
    title: Option<String>,          // Title is required
    file_name: Option<String>,      // File name is required
    extension: Option<String>,      // Extension is required
    artist: Option<String>,         // Artist is optional
    album: Option<String>,          // Album is optional
    cover: Option<String>,          // Cover is optional
    output_path: Option<PathBuf>,   // Output path is optional
}

// An enum that will hold all the reasons anything could fail
#[derive(Debug)]
enum FailReason
{
    StdInFailed
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
    // First, get all the user's arguments as a vector
    let args: Vec<String> = std::env::args().skip(1).collect();

    //
    // Parse the title
    //
    
    // Get the position of the title flag, assign it
    let title_index_opt = args.iter().position(|i| i.as_str() == "-title" || i.as_str() == "-t");
    if let Some(flag_index) = title_index_opt
    {
        if let Some(requested_title) = args.get(flag_index + 1)
        {
            file_data.title = Some(String::from(requested_title));
        }
    }

    // If file_data.title is None, there was either no title flag or the user didn't enter a title
    if file_data.title == None
    {
        // Prompt the user to enter a title
        println!("You didn't enter a title. Please do so now:");
        print!(">>>");
        std::io::stdout().flush().unwrap();
        let mut entered_title = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut entered_title)
        {
            entered_title.pop();
            file_data.title = Some(entered_title);
        } else { return Err(FailReason::StdInFailed); }
    }


    //
    // Parse the file name
    //
    
    // Get the position of the title flag, assign it
    let filename_index_opt = args.iter().position(|i| i.as_str() == "-filename" || i.as_str() == "-n");
    let unallowed_chars = Regex::new(r#"[!@#$%^&*\\\n\t\r"'/?<>:]"#).unwrap();
    if let Some(flag_index) = filename_index_opt
    {
        if let Some(requested_filename) = args.get(flag_index + 1)
        {
            let formatted_filename = unallowed_chars.replace_all(requested_filename, "");
            if requested_filename.as_str() != formatted_filename { println!("Invalid file name - using \"{}\"", formatted_filename); }
            file_data.file_name = Some(formatted_filename.to_string());
        }
    }

    // If file_data.title is None, there was either no title flag or the user didn't enter a title
    if file_data.file_name == None
    {
        // Prompt the user to enter a title
        println!("You didn't enter a file name. Please do so now:");
        print!(">>>");
        std::io::stdout().flush().unwrap();
        let mut entered_filename = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut entered_filename)
        {
            entered_filename.pop();
            let formatted_filename = unallowed_chars.replace_all(entered_filename.as_str(), "");
            if entered_filename.as_str() != formatted_filename { println!("Invalid file name - using \"{}\"", formatted_filename); }
            file_data.file_name = Some(formatted_filename.to_string());
        } else { return Err(FailReason::StdInFailed); }
    }

    Ok(())
}

