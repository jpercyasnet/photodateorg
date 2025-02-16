use rfd::FileDialog;
use std::path::{Path};
use std::fs;
use regex::Regex;

pub fn targetdirpressm (dirval: String) -> (u32, String, String, String) {
     let errcode: u32;
     let errstring: String;
     let mut new_dir: String;
     let mut targetdate = "NONE".to_string();
     if Path::new(&dirval).exists() {
         new_dir = dirval.to_string();
     } else {
         new_dir = "/".to_string();
     }
     let folder = FileDialog::new()
//        .set_location(&new_dir)
//        .show_open_single_dir()
//        .unwrap();
         .set_directory(&new_dir)
         .pick_folder();
     if folder == None {
         errstring = "error getting target directory -- possible cancel key hit".to_string();
         errcode = 1;
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         if Path::new(&new_dir).exists() {
             let mut bolok = true;
             for entry1 in fs::read_dir(&new_dir).unwrap() {
                  let entry = entry1.unwrap();
                  if let Ok(metadata) = entry.metadata() {
                      if let Ok(_file_name) = entry.file_name().into_string() {
                          if metadata.is_file() {
                              bolok = false;
                          }
                      }
                  }
             }
             if bolok {
                 errstring = "target directory selected".to_string();
                 let re = Regex::new(r"(\d{8})").unwrap();
                 if re.is_match(&new_dir) {
                     for cap in re.captures_iter(&new_dir) {
                          targetdate = format!("YYYYMMDD: {}", &cap[1]);
                     }
                 } else {
                     let rea = Regex::new(r"(\d{6})").unwrap();
                     if rea.is_match(&new_dir) {
                         for capa in rea.captures_iter(&new_dir) {
                              targetdate = format!("YYYYMM: {}", &capa[1]);
                         }
                     } else {
                         let reb = Regex::new(r"(\d{4})").unwrap();
                         if reb.is_match(&new_dir) {
                             for capb in reb.captures_iter(&new_dir) {
                                  targetdate = format!("YYYY: {}", &capb[1]);
                             }
                         }
                     }
                 }
                 errcode = 0;
             } else {
                 errstring = "the target directory has files in it".to_string();
                 errcode = 2;
             }
         } else {
             errstring = "error target directory does not exist".to_string();
             errcode = 3;
         }
     } 
     (errcode, errstring, new_dir, targetdate)
}

