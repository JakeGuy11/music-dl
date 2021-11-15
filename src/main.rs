extern crate ytd_rs;
extern crate regex;
use ytd_rs::{YoutubeDL, ResultType, Arg};
use std::process::Command;
use std::path::PathBuf;
use std::io::Write;
use regex::Regex;

// A struct that will hold all the data about the song
#[derive(Debug)]
struct FileData
{
    url: Option<String>,            // URL is required
    title: Option<String>,          // Title is required
    file_name: Option<String>,      // File name is required
    extension: Option<String>,      // Extension is optional
    artist: Option<String>,         // Artist is required
    album: Option<String>,          // Album is optional
    year: Option<String>,           // Year is optional
    track: Option<String>,          // Track is optional
    cover: Option<String>,          // Cover is optional
    output_path: Option<PathBuf>,   // Output path is optional
}

// An enum that will hold all the reasons anything could fail
#[derive(Debug)]
enum FailReason
{
    NoURLProvided,
    StdInFailed,
    YoutubeDLFailed
}

fn main() 
{
    // Create the file data that we'll work with throughout
    let mut song: FileData = FileData
    {
        url: None,
        title: None,
        file_name: None,
        extension: None,
        artist: None,
        album: None,
        year: None,
        track: None,
        cover: None,
        output_path: None
    };
    
    // Parse all the user's cli flags, ask for any we still need
    match parse_flags(&mut song)
    {
        Err(FailReason::NoURLProvided) => { eprintln!("You must provide a URL to download!"); std::process::exit(1); },
        Err(FailReason::StdInFailed) => { eprintln!("Failed to read console input!"); std::process::exit(1); },
        Err(FailReason::YoutubeDLFailed) => { eprintln!("Failed to run youtube-dl command!"); std::process::exit(1); },
        Ok(_) => { println!("Downloading..."); },
    }

    // Generate the ffmpeg download command
    let command = generate_ffmpeg_flags(&song);

    // Now we have the command - execute it and handle errors
    let mut download_command = Command::new("sh");
    download_command.arg("-c").arg(command.as_str());

    match download_command.output()
    {
        Err(_) => { eprintln!("Failed to execute FFMPEG command!"); },
        Ok(_) => {  }
    }
}


//
// Parse the user's cli flags to get all the needed info
//
fn parse_flags(file_data: &mut FileData) -> Result<(), FailReason>
{
    // First, get all the user's arguments as a vector
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Check if the user wants help
    if args.iter().any(|i| i.as_str() == "-h" || i.as_str() == "-help" || i.as_str() == "--help") 
    {
        let help_msg = include_bytes!("help.txt");
        print!("{}", String::from_utf8_lossy(help_msg));
        std::process::exit(0);
    }

    // Check to make sure the user didn't pass their own url
    let cover_ext_url_opt = if !args.iter().any(|i| i.as_str() == "-stream" || i.as_str() == "-s")
    {
        // Get the URL
        let raw_url = if let Some(given_url) = args.last() { String::from(given_url) } else { return Err(FailReason::NoURLProvided); };
        // Turn that raw url into the raw video url
        let downloader = YoutubeDL::new(&PathBuf::from("./"), vec![Arg::new_with_arg("-f", "bestaudio"), Arg::new("-g")], raw_url.as_str()).unwrap().download();
        file_data.url = match downloader.result_type()
        {
            ResultType::SUCCESS => { Some(downloader.output().to_string()) },
            _ => { eprintln!("{}", downloader.output()); return Err(FailReason::YoutubeDLFailed); }
        };
        Some(raw_url)
    }
    else
    {
        // The user passed their own stream - take that
		let stream_index_opt = args.iter().position(|i| i.as_str() == "-stream" || i.as_str() == "-s");
        if let Some(flag_index) = stream_index_opt
        {
            if let Some(requested_stream) = args.get(flag_index + 1)
            {
                file_data.url = Some(String::from(requested_stream));
            }
        }
        if file_data.year == None
        {
			eprintln!("You must provide an audio/visual stream!");
			std::process::exit(1);
        }
        None
    };


    //
    // Parse the cover, make sure the url is valid
    //
    
    // Get the position of the cover flag, assign it
    let cover_index_opt = args.iter().position(|i| i.as_str() == "-cover" || i.as_str() == "-c");
    if let Some(flag_index) = cover_index_opt
    {
        if let Some(requested_cover) = args.get(flag_index + 1)
        {
            file_data.cover = Some(String::from(requested_cover));
        }
    }
    if file_data.cover == None
    {
        if let Some(cover_ext_url) = cover_ext_url_opt
        {
            // Get the youtube thumbnail
            let extraction_url = cover_ext_url;
            let dl_res = YoutubeDL::new(&PathBuf::from("./"), vec![Arg::new("--get-thumbnail")], extraction_url.as_str()).unwrap().download();
            let mut cover_url = String::from(dl_res.output());
            cover_url.pop();
            file_data.cover = Some(cover_url);
            println!("Non-youtube audio stream provided: using youtube URL for cover image only");
        }
    }


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
            if requested_filename.as_str() != formatted_filename { println!("Invalid file name: using \"{}\"", formatted_filename); }
            file_data.file_name = Some(formatted_filename.to_string());
        }
    }

    // If file_data.title is None, there was either no title flag or the user didn't enter a title
    if file_data.file_name == None
    {
        let copied_title = file_data.title.clone().unwrap();
        let formatted_filename = unallowed_chars.replace_all(copied_title.as_str(), "");
        file_data.file_name = Some(formatted_filename.to_string());
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
    // Parse the year
    //
    
    // Get the index of the year tag
    if !args.iter().any(|i| i.as_str() == "-no-year" || i.as_str() == "-ny")
    {
        let year_index_opt = args.iter().position(|i| i.as_str() == "-year" || i.as_str() == "-y");
        if let Some(flag_index) = year_index_opt
        {
            if let Some(requested_year) = args.get(flag_index + 1)
            {
                file_data.year = Some(String::from(requested_year));
            }
        }
        if file_data.year == None
        {
            // Prompt the user to enter a year
            println!("You didn't enter a year. Please do so now:");
            print!(">>>");
            std::io::stdout().flush().unwrap();
            let mut entered_year = String::new();
            if let Ok(_) = std::io::stdin().read_line(&mut entered_year)
            {
                entered_year.pop();
                if entered_year != String::from("") { file_data.year = Some(entered_year); }
            } else { return Err(FailReason::StdInFailed); }
        }
    }

    //
    // Parse the track number
    //
    
    // Get the index of the track tag
    if !args.iter().any(|i| i.as_str() == "-no-track" || i.as_str() == "-nt")
    {
        let track_index_opt = args.iter().position(|i| i.as_str() == "-track" || i.as_str() == "-r");
        if let Some(flag_index) = track_index_opt
        {
            if let Some(requested_track) = args.get(flag_index + 1)
            {
                file_data.track = Some(String::from(requested_track));
            }
        }
        if file_data.track == None
        {
            file_data.track = Some(String::from("1"));
        }
    }

    Ok(())
}

fn generate_ffmpeg_flags(file_data: &FileData) -> String
{
    let mut command: Vec<String> = Vec::new();

    // This is the format of the ffmpeg command:
    // ffmpeg -i youtube_song_url -i youtube_cover_url -map 0:0 -map 1:0 -q:a 0 -y -id3v2_version 3
    //      -metadata title="my_title" -metadata album="album_name" -metadata artist="artist_name"
    //      -metadata date="year" -metadata track="track#" -metadata:s:v comment="Cover
    //      (front)" output_file.mp3
    // And I'm just now realizing that I need to add options for the year and track :'(

    // First, add all the mandatory stuff
    command.push(String::from("ffmpeg"));
    command.push(String::from("-i"));
    command.push(String::from(format!("\"{}\"", file_data.url.as_ref().unwrap())));
    command.push(String::from("-i"));
    command.push(String::from(format!("\"{}\"", file_data.cover.as_ref().unwrap())));
    command.push(String::from("-map 0:0 -map 1:0"));
    command.push(String::from("-q:a 0 -y"));
    command.push(String::from("-id3v2_version 3"));

    // Now add the metadata
    // Add the title
    command.push(format!("-metadata title=\"{}\"", file_data.title.as_ref().unwrap()));
    // Add the artist
    command.push(format!("-metadata artist=\"{}\"", file_data.artist.as_ref().unwrap()));
    // Add the album if it's not none
    if let Some(album) = file_data.album.as_ref() { command.push(format!("-metadata album=\"{}\"", album)); }
    // Add the year (implement later)
    if let Some(year) = file_data.year.as_ref() { command.push(format!("-metadata date=\"{}\"", year)); }
    // Add the track number (implement later)
    if let Some(track) = file_data.track.as_ref() { command.push(format!("-metadata track=\"{}\"", track)); }
    // Add the cover details
    command.push(format!("-metadata:s:v comment=\"Cover (front)\""));

    // Finally, add the output file
    command.push(format!("\"{}/{}.{}\"", file_data.output_path.as_ref().unwrap().to_str().unwrap(), file_data.file_name.as_ref().unwrap(), file_data.extension.as_ref().unwrap()));

    // Concat all the commands and return it
    command.join(" ")
}

