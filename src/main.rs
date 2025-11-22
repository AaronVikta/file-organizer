use std::{collections::HashMap, fs, io, path::Path};

use clap::{Parser, Subcommand};

#[derive(Debug,Parser)]
#[command(name = "File Organizer",version="1.0")]
struct  Cli{
    // Source directory to organize
    #[command(subcommand)]
    command: Actions

}

#[derive(Debug, Subcommand)]
enum Actions{
    Organize{
        /// Source directory to organize
        #[arg(short, long)]
        source: String,

        #[arg(short,long)]
        dry_run:bool,
    },
    List{
        #[arg(short,long)]
        source:String,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command{
        Actions::Organize{source,dry_run}=> {
            println!("Organizing files in directory: {}", source);
            // Add file organization logic here
            if dry_run{
                println!("You are running in dry run mode - No changes will be mader");
            }

            match organize_files(&source, dry_run){
                Ok(count)=>{
                    if dry_run{
                        println!("Would organize {} files",count);
                    }
                    else{
                        println!("Successfully organized {} files", count);
                    }
                }
                Err(e)=>{
                    eprintln!("Error organizing files: {}", e);
                }
            }
        }
        Actions::List { source }=>{
            println!("Listing files in directory: {}", source);
            match list_files(&source){
                Ok(_)=>{}
                Err(e)=>eprintln!("Error listing {}", e)
            }
        }
    }
}


fn get_category(extension: &str)->&str{
    match extension.to_lowercase().as_str(){
         "jpg"|"jpeg"|"png"|"gif"|"bmp"| "svg"|"webp"|"tiff"|"ico" =>"Images",

         //Documents 
         "pdf"|"doc"|"docx"|"txt"|"rtf"|"odt"|"tex" => "Documents",

         //Spreadsheets
            "xls"|"xlsx"|"ods"|"csv" => "Spreadsheets",

            //Presentations
            "ppt"|"pptx"|"odp" => "Presentations",

            //Videos
            "mp4"|"mkv"|"flv"|"avi"|"mov"|"wmv"|"webm" => "Videos",

            //Audio
            "mp3"|"wav"|"flac"|"aac"|"ogg"|"wma"|"m4a" =>"Audio",

            //Archives
            "zip"|"rar"|"7z"|"tar"|"gz"|"bz2" => "Archives",

            //Code
            "rs"|"py"|"js"|"java"|"c"|"cpp"|"php"|"swift"|"ts"|"go" => "Code",

            //Web
            "html"|"css"|"json"|"yaml"|"yml"|"xml" => "Web",

            //Executables
            "exe"|"msi"|"app"|"deb"|"rpm" => "Executables",
            _ => "Others",
    }
   
}

fn organize_files(source_dir: &str, dry_run: bool)->io::Result<usize>{
    let source_path = Path::new(source_dir);

    if !source_path.exists(){
        return  Err(io::Error::new(
            io::ErrorKind::NotFound, 
            "Source Directory does not exist"));
    }

    if !source_path.is_dir(){
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source path is not a directory"));
    }

    let entries = fs::read_dir(source_path)?;
    let mut moved_count = 0;

    for entry in entries{
        let entry = entry?;

        let path = entry.path();

        //skip directories
        if path.is_dir(){
            continue;
        }

        //Get file extension
        if let Some(extension) =path.extension(){
            if let Some(ext_str) = extension.to_str(){
                let category = get_category(ext_str);

                let category_dir = source_path.join(category);

                if !dry_run && !category_dir.exists(){
                    fs::create_dir(&category_dir)?;
                }
                
                let file_name = path.file_name().expect("Failed to get file name");
                let dest_path = category_dir.join(file_name);

                if dry_run{
                    println!("{}->{}/{}", 
                path.display(),
            category,
        file_name.to_string_lossy());
                
                }
                else {
                    match fs::rename(&path, &dest_path){
                        Ok(_) =>{
                            println!("Moved {} to {}/{}",
                        path.file_name().expect("File was not moved").to_string_lossy(),
                    category,
                file_name.to_string_lossy());
            
                        }
                        Err(e)=>{
                            eprintln!("Failed to move {}: {}",
                        path.display(),
                    e);
                    continue;
                        }
                    }
                }
                moved_count +=1;
            }
        }

        else {
            let category = "Others";
            let category_dir = source_path.join(category);

            if !dry_run && !category_dir.exists(){
                fs::create_dir(&category_dir)?;
            }

            let file_name= path.file_name().expect("Failed to get file name");
            let dest_path  = category_dir.join(file_name);

            if dry_run{
                println!("{} -> {}/{}",
            path.display(),
        category,
    file_name.to_string_lossy());
            } else{
                match fs::rename(&path, &dest_path){
                    Ok(_)=>{
                        println!("Moved {} -> {}/{}",
                    path.file_name().expect("Error Moving Files").to_string_lossy(),
                category,
            file_name.to_string_lossy());
                    }
                    Err(e)=>{
                        eprintln!("Failed to move {}:{}", path.display(),e);
                        continue;
                    }
                }
            }
            moved_count +=1;
        }
    }
    Ok(moved_count)
}



fn list_files(source_dir: &str)->io::Result<()>{
    let source_path = Path::new(source_dir);

    if !source_path.exists(){
        return Err(io::Error::new(io::ErrorKind::NotFound, 
            "Source directory does not exist"));
    }

    let entries  = fs::read_dir(source_path)?;

    let mut category_map: HashMap<String,Vec<String>> = HashMap::new();

    for entry in entries{
        let entry  = entry?;
        let path =  entry.path();

        if path.is_dir(){
            continue;
        }

        let file_name = path.file_name().expect("Failed toget file name").to_string_lossy().to_string();

        if let Some(extension) = path.extension(){
            if let Some(ext_str) = extension.to_str(){
                let category = get_category(ext_str).to_string();

                category_map.entry(category).or_insert_with(Vec::new).push(file_name);                
            }
        }
        else {
            category_map.entry("Others".to_string()).or_insert_with(Vec::new).push(file_name);

        }
    }
    for (category,files) in category_map.iter(){
        println!("{}({} files)",category,files.len());

        for file in files{
            println!("{}", file);
        }
        println!()
    }
    Ok(())
}