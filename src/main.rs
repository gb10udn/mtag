use lofty::file::TaggedFileExt;
use lofty::read_from_path;
use lofty::tag::{Accessor, TagExt};
use lofty::config::WriteOptions;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'a', long)]
    artist: Option<String>,

    #[arg(short = 'b', long)]
    album: Option<String>,

    #[arg(value_name = "FILES", default_value = "-", num_args = 1.., trailing_var_arg = true)]
    files: Vec<String>,
}


fn change_tag(path: &str, artist: Option<&str>, album: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tagged_file = read_from_path(path)?;
    let primary_tag = tagged_file.primary_tag_mut();
    let primary_tag = primary_tag.unwrap();

	if let Some(artist) = artist {
		primary_tag.set_artist(artist.into());
	} else {
        primary_tag.remove_artist();
    }

	if let Some(album) = album {
		primary_tag.set_album(album.into());
	} else {
        primary_tag.remove_album();
    }

    Ok(primary_tag.save_to_path(path, WriteOptions::default())?)
}

 
fn main() {
    let args = Args::parse();
    for path in args.files {
        change_tag(&path, args.artist.as_deref(), args.album.as_deref()).unwrap();
    }
}