use regex::Regex;
use chrono::offset::LocalResult;
use chrono::prelude::*;
//use chrono::{Duration, Utc};
use std::process::Command as stdCommand;
use std::path::Path;

pub fn executechangepressm (listfiles: Vec<String>, sourcedir: String, targetdir: String)
           -> (u32, String, Vec<String>) {
     let mut errcode: u32 = 0;
     let mut errstring: String = "inital value".to_string();
     let mut newlist: Vec<String> = Vec::new();
     let mut bolok = false;
     let mut numprocess = 0;
     if Path::new(&sourcedir).exists() {
         if Path::new(&targetdir).exists() {
             if sourcedir == targetdir {
                 errstring = "Error: Source dir equals Target dir".to_string();
                 errcode = 1;
             } else {
                 bolok = true;
             }
         } else {
             errstring = "Error: Target dir does not exist".to_string();
             errcode = 2;
         }
     } else {
         errstring = "Error: Source dir does not exist".to_string();
         errcode = 3;
     }
     if bolok {
//         println!("listfiles length {}", listfiles.len());
         if listfiles.len() < 1 {
             errstring = "Error: list of files is less than 1".to_string();
             errcode = 4;
             bolok = false;
         }
     }
     if bolok {
         let mut linenum = 0;
         for itemall in listfiles {
              linenum = linenum + 1;
              let itemallvec: Vec<&str> = itemall.split("|").collect();
              if itemallvec.len() < 6 {
                  errstring = format!("Error line number {}: list length less than 6: {}", linenum, itemallvec.len());
                  errcode = 5;
                  break;
              }
              let filenamex = itemallvec[1].trim();
              let mut filelocx = itemallvec[2].trim();
              let filecurx = itemallvec[3].trim();
              let fileassx = itemallvec[4].trim();
              let filenewdir = itemallvec[5].trim();
//              println!("line num {} filelocx -{}-", linenum, filelocx);
              if filelocx == "Date Chg" {
                  let fullfrom = sourcedir.clone() + "/" + &filenamex;
//                  let fullfrom = format!("{}/{}", sourcedir, filenamex);
                  if !Path::new(&fullfrom).exists() {
                      errstring = format!("Error line number {}: source file does not exist: -{}-", linenum, fullfrom);
                      errcode = 5;
                      break;
                  }
                  let fulldirto = format!("{}/{}", targetdir, filenewdir);
                  let fullto = format!("{}/{}", fulldirto, filenamex);
                  if Path::new(&fullto).exists() {
                      errstring = format!("Error line number {}: target file already exists: {}", linenum, fullto);
                      errcode = 6;
                      break;
                  }
                  let mut dateyr = 0;
                  let mut datemo = 0;
                  let mut dateday = 0;
                  let mut datehr = 0;
                  let mut datemin = 0;
                  let mut datesec = 0;
                  let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
                  let after = re.replace_all(&fileassx, "_");
                  let listdatex: Vec<&str> = after.split("_").collect();
                  let lendat2 = listdatex.len();
                  let mut baddate1 = false;
                  for indl in 0..lendat2 {
                       let date_int: i32 = listdatex[indl].parse().unwrap_or(-9999);
                       if date_int == -9999 {
                           baddate1 = true;
                           break;
                       } else {
                           match indl {
                              0 => dateyr = date_int,
                              1 => datemo = date_int as u32,
                              2 => dateday = date_int as u32,
                              3 => datehr = date_int as i32,
                              4 => datemin = date_int as i32,
                              5 => datesec = date_int as i32,
                              _ => baddate1 = true,
                           }
                       }
                  }
                  if !baddate1 {
                      let datexx = Local.with_ymd_and_hms(dateyr, datemo, dateday,1,1,1);
                      if datexx == LocalResult::None {
                          baddate1 = true;
                      } else {
                          if (datehr < 0) | (datehr > 23) {
                              baddate1 = true;
                          } else if (datemin < 0) | (datemin > 59) {
                              baddate1 = true;
                          } else if (datesec < 0) | (datesec > 59) {
                              baddate1 = true;
                          }
                      }
                  }
                  if baddate1 {
                      errstring = format!("Error line number {}: bad assigned date {}", linenum, fileassx);
                      errcode = 7;
                      break;
                  }
                  let timestampx = format!("{}{:0>2}{:0>2}{:0>2}{:0>2}.{:0>2}", dateyr, datemo, dateday, datehr, datemin, datesec);
                  if !Path::new(&fulldirto).exists() {
                      let _outmk = stdCommand::new("mkdir")
                                         .arg("-p")
                                         .arg(&fulldirto)
                                         .output()
                                         .expect("failed to execute process");
                  }
                  let _outcp = stdCommand::new("cp")
                                         .arg("-p")
                                         .arg(&fullfrom)
                                         .arg(&fullto)
                                         .output()
                                         .expect("failed to execute process");
                  let _outtouch = stdCommand::new("touch")
                                         .arg("-a")
                                         .arg("-m")
                                         .arg("-t")
                                         .arg(&timestampx)
                                         .arg(&fullto)
                                         .output()
                                         .expect("failed to execute process");

                  numprocess = numprocess + 1;
                  filelocx = "Copied"
              }
              newlist.push(format!("{}|{}|{}|{}|{}", filenamex, filelocx, filecurx, fileassx, filenewdir));
         }
         if errcode == 0 {
             if numprocess < 1 {
                 errstring = format!(r#"Error no lines had "Date Chg""#);
                 errcode = 7;
             } else {
                 errstring = format!("{} lines were copied", numprocess);
             }
         }
//              println!("line num {} fileupdate -{}-  filenamex -{}-  filelocx -{}-  filecurx -{}-  fileassx -{}-  filenamex -{}-", 
//                         linenum, fileupdate, filenamex, filelocx, filecurx, fileassx, filenewnamex);
     } 
     (errcode, errstring, newlist)
}

