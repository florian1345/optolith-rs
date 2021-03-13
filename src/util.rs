use crate::error::{self, OptolithDataResult};

use serde::{Deserialize, Serialize};

use std::ffi::OsString;
use std::fs::{self, DirEntry, File, ReadDir};
use std::io::{Read, Write};
use std::path::PathBuf;

use zip::{CompressionMethod, ZipArchive, ZipWriter};
use zip::write::FileOptions;

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
    error::set_file(&mut res, OsString::from(file.file_name().unwrap()));
    res
}

pub struct UtilReadDir {
    read_dir: ReadDir,
    file: OsString
}

impl Iterator for UtilReadDir {
    type Item = OptolithDataResult<DirEntry>;

    fn next(&mut self) -> Option<OptolithDataResult<DirEntry>> {
        match self.read_dir.next() {
            Some(Ok(d)) => Some(Ok(d)),
            Some(Err(e)) => {
                let mut res = Err(e.into());
                error::set_file(&mut res, self.file.clone());
                Some(res)
            },
            None => None
        }
    }
}

fn read_dir_do(path: &PathBuf) -> OptolithDataResult<UtilReadDir> {
    Ok(UtilReadDir {
        read_dir: fs::read_dir(path)?,
        file: OsString::from(path.file_name().unwrap())
    })
}

pub fn read_dir(path: &PathBuf) -> OptolithDataResult<UtilReadDir> {
    let mut res = read_dir_do(path);
    error::set_file(&mut res, OsString::from(path.file_name().unwrap()));
    res
}

/// Joins paths `p1` and `p2` as strings into a [PathBuf](std::path::PathBuf).
pub fn join(p1: &str, p2: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(p1);
    path_buf.push(p2);
    path_buf
}

pub fn to_compressed_file<T>(t: &T, path: &str) -> OptolithDataResult<()>
where
    T : Serialize
{
    let file = File::create(path)?;
    let mut writer = ZipWriter::new(file);
    let file_options = FileOptions::default().compression_method(CompressionMethod::Bzip2);
    writer.start_file("0", file_options).unwrap();
    writer.write_all(&serde_json::to_vec(t)?)?;
    Ok(())
}

pub fn from_compressed_file<T>(path: &str) -> OptolithDataResult<T>
where
    for<'de> T : Deserialize<'de>
{
    let mut archive = ZipArchive::new(File::open(path).unwrap()).unwrap();
    let mut json = String::new();
    archive.by_name("0").unwrap().read_to_string(&mut json).unwrap();
    Ok(serde_json::from_str::<T>(&json)?)
}
