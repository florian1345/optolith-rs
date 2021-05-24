use crate::error::{self, OptolithDataResult};

use serde::{Deserialize, Serialize};

use std::fs::{self, DirEntry, File, ReadDir};
use std::io::{Read, Write};
use std::path::PathBuf;

fn deserialize_yaml_file_do<T>(file: &PathBuf) -> OptolithDataResult<T>
where
    for<'de> T : Deserialize<'de>
{
    let yaml = fs::read_to_string(file.as_path())?;
    Ok(serde_yaml::from_str::<T>(&yaml)?)
}

/// Reads the file with the given path and parses its content as YAML into an
/// instance of type `T`.
pub fn deserialize_yaml_file<T>(file: &PathBuf) -> OptolithDataResult<T>
where
    for<'de> T : Deserialize<'de>
{
    let mut res = deserialize_yaml_file_do(file);
    error::set_file(&mut res, file);
    res
}

pub struct UtilReadDir<'a> {
    read_dir: ReadDir,
    file: &'a PathBuf
}

impl<'a> Iterator for UtilReadDir<'a> {
    type Item = OptolithDataResult<DirEntry>;

    fn next(&mut self) -> Option<OptolithDataResult<DirEntry>> {
        match self.read_dir.next() {
            Some(Ok(d)) => Some(Ok(d)),
            Some(Err(e)) => {
                let mut res = Err(e.into());
                error::set_file(&mut res, self.file);
                Some(res)
            },
            None => None
        }
    }
}

fn read_dir_do(path: &PathBuf) -> OptolithDataResult<UtilReadDir> {
    Ok(UtilReadDir {
        read_dir: fs::read_dir(path)?,
        file: path
    })
}

pub fn read_dir(path: &PathBuf) -> OptolithDataResult<UtilReadDir> {
    let mut res = read_dir_do(path);
    error::set_file(&mut res, path);
    res
}

/// Joins paths `p1` and `p2` as strings into a [PathBuf](std::path::PathBuf).
pub fn join(p1: &str, p2: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(p1);
    path_buf.push(p2);
    path_buf
}

pub fn to_file<T>(t: &T, path: &str) -> OptolithDataResult<()>
where
    T : Serialize
{
    let mut file = File::create(path)?;
    file.write_all(&serde_json::to_vec(t)?)?;
    Ok(())
}

pub fn from_file<T>(path: &str) -> OptolithDataResult<T>
where
    for<'de> T : Deserialize<'de>
{
    let mut file = File::open(path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    Ok(serde_json::from_str(&json)?)
}
