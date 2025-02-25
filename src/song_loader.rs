use crate::notedata::{self, NoteData};
use bincode::deserialize;
use rayon::{iter::ParallelIterator, prelude::*};
use std::io::Read;
use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

pub fn load_song<T>(simfile_folder: T) -> Option<NoteData>
where
    T: AsRef<Path> + Clone,
{
    let n = load_song_rm(simfile_folder.clone());
    if n.is_some() {
        n
    } else {
        load_song_sm(simfile_folder.clone())
    }
}

pub fn load_song_sm<T>(simfile_folder: T) -> Option<NoteData>
where
    T: AsRef<Path>,
{
    read_dir(simfile_folder)
        .expect("Couldn't open folder")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension() == Some(OsStr::new("sm")))
        .filter_map(|sim| File::open(sim.path()).ok())
        .find_map(|sim| notedata::NoteData::from_sm(sim).ok())
}

pub fn load_song_rm<T>(simfile_folder: T) -> Option<NoteData>
where
    T: AsRef<Path>,
{
    read_dir(simfile_folder)
        .expect("Couldn't open folder")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension() == Some(OsStr::new("rm")))
        .filter_map(|sim| File::open(sim.path()).ok())
        .find_map(|mut sim| {
            let mut n = vec![];
            sim.read_to_end(&mut n).unwrap();
            deserialize(&n).ok()
        })
}

pub fn load_songs_folder<T>(songs_directory: T) -> Vec<(PathBuf, Option<NoteData>)>
where
    T: AsRef<Path>,
{
    get_subfolder_list(songs_directory.as_ref())
        .par_iter()
        .map(|x| (x.clone(), load_song(x)))
        .collect()
}

pub fn get_subfolder_list(songs_folder: &Path) -> Vec<PathBuf> {
    let mut output: Vec<PathBuf> = Vec::new();
    output.push(songs_folder.to_path_buf());
    read_dir(songs_folder)
        .expect("Couldn't open folder")
        .filter_map(|entry| entry.ok())
        .filter(|dir_path| dir_path.path().is_dir())
        .for_each(|dir_entry| output.append(&mut get_subfolder_list(&dir_entry.path())));
    output
}
