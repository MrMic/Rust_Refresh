mod player;

fn main() {
    player::play_movie("The Matrix");
    player::play_audio("Bohemian Rhapsody");

    clean::perform_cleaning();
    clean::files::clean_files();
}

//INFO:          ╭─────────────────────────────────────────────────────────╮
//INFO:          │                      MODULE INSIDE                      │
//INFO:          ╰─────────────────────────────────────────────────────────╯
mod clean {
    pub fn perform_cleaning() {
        println!("Performing cleaning... 🧹");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Cleaning files... 🗂️");
        }
    }
}
