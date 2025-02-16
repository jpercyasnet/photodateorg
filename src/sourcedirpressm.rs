extern crate chrono;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::fs;
use std::io::BufReader;
use chrono::prelude::*;
use chrono::offset::LocalResult;
use rfd::FileDialog;
use regex::Regex;
use exif::{Reader, In, Tag};
use crate::dump_file::dump_file;

pub fn sourcedirpressm(dirval: String, dateinname: bool) -> (u32, String, String, Vec<String>, String) { 
     let mut errcode: u32 = 0;
     let mut errstring = "********* ERROR File format is not correct **********".to_string();
     let mut sourcedate = "NONE".to_string();
     let mut numentry = 0;
     let mut baddate1 = 0;
     let mut listitems: Vec<String> = Vec::new();
     let mut new_dir: String;
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
         errstring = "error getting directory -- possible cancel key hit".to_string();
         errcode = 1;
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         let re = Regex::new(r"(\d{8})").unwrap();
         if re.is_match(&new_dir) {
             for cap in re.captures_iter(&new_dir) {
                  sourcedate = format!("YYYYMMDD: {}", &cap[1]);
             }
         } else {
             let rea = Regex::new(r"(\d{6})").unwrap();
             if rea.is_match(&new_dir) {
                 for capa in rea.captures_iter(&new_dir) {
                      sourcedate = format!("YYYYMM: {}", &capa[1]);
                 }
             } else {
                 let reb = Regex::new(r"(\d{4})").unwrap();
                 if reb.is_match(&new_dir) {
                     for capb in reb.captures_iter(&new_dir) {
                          sourcedate = format!("YYYY: {}", &capb[1]);
                     }
                 }
             }
         }
         let current_dir = PathBuf::from(&new_dir);
         let mut date_from;
         let mut file_date;
         for entry1 in fs::read_dir(&current_dir).unwrap() {
              let entry = entry1.unwrap();
              if let Ok(metadata) = entry.metadata() {
                  if let Ok(file_name) = entry.file_name().into_string() {
                      if metadata.is_file() {
                          if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                             file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") {
                              if dateinname {
                                  let dateto;
                                  let mut dateyr = 0;
                                  let mut datemo = 0;
                                  let mut dateday = 0;
                                  let mut datehr = 0;
                                  let mut datemin = 0;
                                  let mut datesec = 0;
                                  let mut datenum = 0;
                                  if file_name.len() < 25 {
                                      baddate1 = 1;
                                  } else {
// date in name start
                                      let date1ar2: Vec<&str> = file_name[0..23].split("_").collect();
                                      let lendat2 = date1ar2.len();
                                      for indl in 0..lendat2 {
                                           let date_int: i32 = date1ar2[indl].parse().unwrap_or(-9999);
                                           if date_int == -9999 {
                                               baddate1 = 1;
                                               break;
                                           } else {
                                               match indl {
                                                 0 => dateyr = date_int,
                                                 1 => datemo = date_int as u32,
                                                 2 => dateday = date_int as u32,
                                                 3 => datehr = date_int as i32,
                                                 4 => datemin = date_int as i32,
                                                 5 => datesec = date_int as i32,
                                                 6 => datenum = date_int as i32,
                                                 _ => baddate1 = 1,
                                               }
                                           }
                                      }
                                  }
                                  if baddate1 == 0 {
                                      let datexx = Local.with_ymd_and_hms(dateyr, datemo, dateday, 0, 0, 0);
                                      if datexx == LocalResult::None {
                                          baddate1 = 1;
                                      } else {
                                          if (datenum < 0) | (datenum > 999) {
                                              baddate1 = 1;
                                          } else if (datehr < 0) | (datehr > 23) {
                                              baddate1 = 1;
                                          } else if (datemin < 0) | (datemin > 59) {
                                              baddate1 = 1;
                                          } else if (datesec < 0) | (datesec > 59) {
                                              baddate1 = 1;
                                          }
                                      }
                                  }
// date in name end
//                          add the mod date values 
                                  if baddate1 == 0 {
                                      dateto = Utc.with_ymd_and_hms(dateyr, datemo, dateday, datehr.try_into().unwrap(), datemin.try_into().unwrap(), datesec.try_into().unwrap()).unwrap();
                                      file_date = format!("{}", dateto.format("%Y-%m-%d %T"));
                                      date_from = format!("date in name");
                                  } else {
                                      errstring = "error ********* BAD date in Name **********".to_string();
                                      errcode = 2;
                                      break;
                                  }
                              } else {
                                  let datetime: DateTime<Local> = metadata.modified().unwrap().into();
                                  file_date = format!("{}", datetime.format("%Y-%m-%d %T"));
                                  date_from = format!("file date");
                                  let file_path = entry.path();
                                  if let Err(_e) = dump_file(&file_path) {
//                                                  orient = format!("Meta error : {}", e);
                                  } else {
                                      let file = File::open(file_path).unwrap();
                                      let reader = Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                                      if let Some(field1) = reader.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                                          file_date = format!("{}",field1.value.display_as(field1.tag));
                                          date_from = format!("date taken");
                                      } else {
                                          if let Some(field2) = reader.get_field(Tag::DateTime, In::PRIMARY) {
                                              file_date = format!("{}",field2.value.display_as(field2.tag));
                                              date_from = format!("image date");
                                          }
                                      }
                                  }
                              }
                              let listival = file_name + "|" + &date_from + "|" + &file_date + "|-|-";
                              listitems.push(listival);
                              numentry = numentry + 1;
                                       
                          }
                      }
                  }
              }
         }
         if baddate1 == 0 {
             if numentry > 0 {
                 listitems.sort();
                 errstring = format!("{} files in directory ", numentry);
                 errcode = 0;
             } else {
                 errstring = format!("********* Directory 1: directory has no images  **********");
                 errcode = 3;
             }              
         }
     }
     (errcode, errstring, new_dir, listitems, sourcedate)
}

