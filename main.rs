use std::env::{args, Args};
use std::fs;
use std::path::Path;

const ENDING_TOKEN: Option<char> = Some('~');

fn main() {
    let mut removed_files_counter: usize = 0;
    
    let mut args: Args = args();
    let input_drop_in: String = args.nth(1).expect("Drop-in point cannot be read from commandline input!");  
    let drop_in = Path::new(&input_drop_in);

    check_dir(drop_in, &mut removed_files_counter);
    println!("Files removed: {}", removed_files_counter);
}

fn check_dir(drop_in: &Path, counter: &mut usize) {
    let entries_iterator = fs::read_dir(drop_in);
    if entries_iterator.is_err() {
	println!("Could not open {:?}", drop_in);
	return
    }
    let entries_iterator = entries_iterator.unwrap();
    for e in entries_iterator {
	let e = e.unwrap();
	let e_is_dir = e.file_type().expect("Error unwrapping file type!").is_dir();
	if e_is_dir {
	    check_dir(&e.path(), counter);
	} else {
	    let e_name = e.file_name().into_string().expect("Could not parse filename!");
	    let mut e_name_as_chars = e_name.chars();

	    if e_name_as_chars.next_back() == ENDING_TOKEN {
		fs::remove_file(e.path().as_path());
		*counter += 1;
	    }
	}
    }
}
