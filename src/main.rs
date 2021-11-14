extern crate ytd_rs;
extern crate regex;
extern crate reqwest;
extern crate image;
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
    extension: Option<String>,      // Extension is optional
    artist: Option<String>,         // Artist is required
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

    //
    // Parse the extension
    //
    
    // Get the position of the extension flag, assign it
    let ext_index_opt = args.iter().position(|i| i.as_str() == "-extension" || i.as_str() == "-e");
    if let Some(flag_index) = ext_index_opt
    {
        if let Some(requested_ext) = args.get(flag_index + 1)
        {
            file_data.extension = Some(String::from(requested_ext));
        }
    }

    // If file_data.title is None, there was either no title flag or the user didn't enter a title
    if file_data.extension == None { file_data.extension = Some(String::from("mp3")) }


    //
    // Parse the path
    //
    
    // Get the position of the path flag, assign it
    let path_index_opt = args.iter().position(|i| i.as_str() == "-path" || i.as_str() == "-p");
    if let Some(flag_index) = path_index_opt
    {
        if let Some(requested_path) = args.get(flag_index + 1)
        {
            file_data.output_path = Some(PathBuf::from(requested_path));
        }
    }

    // If file_data.output_path is None, set it to the default (./)
    if file_data.output_path == None
    {
        file_data.output_path = Some(PathBuf::from("./"));
    }


    //
    // Parse the artist
    //
    
    // Get the position of the artist flag, assign it
    let artist_index_opt = args.iter().position(|i| i.as_str() == "-artist" || i.as_str() == "-a");
    if let Some(flag_index) = artist_index_opt
    {
        if let Some(requested_artist) = args.get(flag_index + 1)
        {
            file_data.artist = Some(String::from(requested_artist));
        }
    }

    // If file_data.title is None, there was either no title flag or the user didn't enter a title
    if file_data.artist == None
    {
        // Prompt the user to enter a title
        println!("You didn't enter an artist. Please do so now:");
        print!(">>>");
        std::io::stdout().flush().unwrap();
        let mut entered_artist = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut entered_artist)
        {
            entered_artist.pop();
            file_data.artist = Some(entered_artist);
        } else { return Err(FailReason::StdInFailed); }
    }


    //
    // Parse the album
    //
    
    // If the user forces no album tag, ignore this whole part and leave it as None
    if !args.iter().any(|i| i.as_str() == "-no-album" || i.as_str() == "-nb")
    {
        // Get the index of the album tag
        let album_index_opt = args.iter().position(|i| i.as_str() == "-album" || i.as_str() == "-b");
        if let Some(flag_index) = album_index_opt
        {
            if let Some(requested_album) = args.get(flag_index + 1)
            {
                file_data.album = Some(String::from(requested_album));
            }
        }

        // If file_data.title is None, there was either no title flag or the user didn't enter a title
        if file_data.album == None
        {
            file_data.album = file_data.title.clone();
        }
    }


    //
    // Parse the cover
    //
    
    // Get the position of the cover flag, assign it
    let mut cover_url = String::new();
    let cover_index_opt = args.iter().position(|i| i.as_str() == "-cover" || i.as_str() == "-c");
    if let Some(flag_index) = cover_index_opt
    {
        if let Some(requested_cover) = args.get(flag_index + 1)
        {
            cover_url = String::from(requested_cover);
        }
    }
    if cover_url == ""
    {
        // Prompt the user to enter a title
        println!("You didn't enter the URL for a cover. Please do so now:");
        print!(">>>");
        std::io::stdout().flush().unwrap();
        let mut entered_cover = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut entered_cover)
        {
            entered_cover.pop();
            cover_url = String::from(entered_cover);
        } else { return Err(FailReason::StdInFailed); }
    }
    
    // Now we have the url, get the image bytes
    let img_bytes = reqwest::blocking::get(cover_url.as_str()).unwrap().bytes().unwrap();
    
    // Turn the bytes into an image and save it
    let cover_image = image::load_from_memory(&img_bytes).unwrap();
    let save_res = cover_image.save_with_format("./out.png", image::ImageFormat::Png);

    Ok(())
}

