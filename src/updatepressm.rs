use regex::Regex;
use chrono::offset::LocalResult;
use chrono::prelude::*;
use chrono::{Duration, Utc};

pub fn updatepressm (all_bool: bool, listfiles: Vec<String>, _sourcedir: String, _targetdir: String, sourcedd_bool: bool,
               sourcedd_value: String, targetdd_bool: bool, targetdd_value: String, hhmmss1_value: String, dateinname_bool: bool)
           -> (u32, String, Vec<String>) {
     let mut errcode: u32 = 0;
     let mut errstring: String = "okxxx".to_string();
     let mut newlist: Vec<String> = Vec::new();
     let mut bolok = true;
     let mut offsettest = false;
     let mut year_int: i32 = 0;
     let mut month_int: i32 = 0;
     let mut day_int: i32 = 0;
     let mut year_set = false;
     let mut month_set = false;
     let mut day_set = false;
     if sourcedd_bool {
         if targetdd_bool {
             errstring = "Error: Check Box for both Source and Target Dir Date are set".to_string();
             errcode = 1;
             bolok = false;
         } else {
             if sourcedd_value.len() < 18 {
                 if sourcedd_value.len() < 14 {
                     if sourcedd_value.len() < 10 {
                         errstring = format!("Error: Invalid value in Source Dir Date: {}", sourcedd_value);
                         errcode = 2;
                         bolok = false;
                     } else {
                         year_int = sourcedd_value[6..10].parse().unwrap_or(-99);
//                         offsettest = true;
                         year_set = true;
                     }
                 } else {
                     year_int = sourcedd_value[8..12].parse().unwrap_or(-99);
                     month_int = sourcedd_value[12..14].parse().unwrap_or(-99);
//                     offsettest = true;
                     year_set = true;
                     month_set = true;
                 }                                  
             } else {
                 year_int = sourcedd_value[10..14].parse().unwrap_or(-99);
                 month_int = sourcedd_value[14..16].parse().unwrap_or(-99);
                 day_int = sourcedd_value[16..18].parse().unwrap_or(-99);
//                 offsettest = true;
                 year_set = true;
                 month_set = true;
                 day_set = true;
             }
         }
     } else {
         if targetdd_bool {
             if targetdd_value.len() < 18 {
                 if targetdd_value.len() < 14 {
                     if targetdd_value.len() < 10 {
                         errstring = format!("Error: Invalid value in Target Dir Date: {}", targetdd_value);
                         errcode = 3;
                         bolok = false;
                     } else {
                         year_int = targetdd_value[6..10].parse().unwrap_or(-99);
//                         offsettest = true;
                         year_set = true;
                     }
                 } else {
                     year_int = targetdd_value[8..12].parse().unwrap_or(-99);
                     month_int = targetdd_value[12..14].parse().unwrap_or(-99);
//                     offsettest = true;
                     year_set = true;
                     month_set = true;
                 }                                  
             } else {
                 year_int = targetdd_value[10..14].parse().unwrap_or(-99);
                 month_int = targetdd_value[14..16].parse().unwrap_or(-99);
                 day_int = targetdd_value[16..18].parse().unwrap_or(-99);
//                 offsettest = true;
                 year_set = true;
                 month_set = true;
                 day_set = true;
             }
         }
     }
     if bolok {
         if year_set {
             if year_int > 0 {
                 if (year_int < 1900) | (year_int > 2100) {
                     errstring = format!("Error: invalid year for dir date. Must be between 1900 and 2100: {}", year_int);
                     errcode = 4;
                     bolok = false;
                 }
             } else if year_int == -99 {
                 errstring = format!("Error: invalid year for dir date. Must be an integer");
                 errcode = 5;
                 bolok = false;
             } else {
                 errstring = format!("Error: invalid year for dir date. Not a positive integer");
                 errcode = 6;
                 bolok = false;
             }
         }
     }
     if bolok {
         if month_set {
             if month_int > 0 {
                 if (month_int < 1) | (month_int > 12) {
                     errstring = format!("Error: invalid month for dir date. Must be between 1 and 12: {}", month_int);
                     errcode = 7;
                     bolok = false;
                 }
             } else if month_int == -99 {
                 errstring = format!("Error: invalid month for dir date. Must be an integer");
                 errcode = 8;
                 bolok = false;
             } else {
                 errstring = format!("Error: invalid month for dir date. Not a positive integer");
                 errcode = 9;
                 bolok = false;
             }
         }
     }
     if bolok {
         if day_set {
             if day_int > 0 {
                 if (day_int < 1) | (day_int > 31) {
                     errstring = format!("Error: invalid day for dir date. Must be between 1 and 31: {}", day_int);
                     errcode = 10;
                     bolok = false;
                 } else {
                     let datexx = Local.with_ymd_and_hms(year_int, month_int as u32, day_int as u32,1,1,1);
                     if datexx == LocalResult::None {
                         errstring = format!("Error: invalid day for dir date month: {} day: {}", month_int, day_int);
                         errcode = 11;
                         bolok = false;
                     }
                 }
             } else if day_int == -99 {
                 errstring = format!("Error: invalid day for dir date. Must be an integer");
                 errcode = 12;
                 bolok = false;
             } else {
                 errstring = format!("Error: invalid day for dir date. Not a positive integer");
                 errcode = 13;
                 bolok = false;
             }
         }
     }
     let mut dateyr1 = 0;
     let mut datemo1 = 0;
     let mut dateday1 = 0;
     let mut datehr1 = 0;
     let mut datemin1 = 0;
     let mut datesec1 = 0;
     if bolok {
// validate offset values
         let datemod1_textx: &String = &format!("{}", hhmmss1_value);
         let date1ar1: Vec<&str> = datemod1_textx[0..].split(":").collect();
         let lendat1 = date1ar1.len();
         if (lendat1 > 6) | (lendat1 < 6) {
             bolok = false;
         } else {
             for indl in 0..lendat1 {
                  let date_int: i32 = date1ar1[indl].parse().unwrap_or(-9999);
                  if date_int == -9999 {
                      bolok = false;
                  } else {
                      match indl {
                         0 => dateyr1 = date_int as i64,
                         1 => datemo1 = date_int as i64,
                         2 => dateday1 = date_int as i64,
                         3 => datehr1 = date_int as i64,
                         4 => datemin1 = date_int as i64,
                         5 => datesec1 = date_int as i64,
                         _ => bolok = false,
                      }
                   }
             }
         }
         if !bolok {
             errstring = format!("Error: Date offset is not formatted correctly: {}", hhmmss1_value);
             errcode = 14;
         } else {
             if dateyr1 != 0 {
                 offsettest = true;
             } else if datemo1 != 0 {
                 offsettest = true;
             } else if dateday1 != 0 {
                 offsettest = true;
             } else if datehr1 != 0 {
                 offsettest = true;
             } else if datemin1 != 0 {
                 offsettest = true;
             } else if datesec1 != 0 {
                 offsettest = true;
             }
         }
     }
     if bolok {
         let mut linenum = 0;
         for itemall in listfiles {
              linenum = linenum + 1;
              let itemallvec: Vec<&str> = itemall.split("|").collect();
              if itemallvec.len() < 6 {
                  errstring = format!("Error line number {}: list length less than 6: {}", linenum, itemallvec.len());
                  errcode = 18;
                  break;
              }
              let mut fileupdate: bool = false;
              if itemallvec[0] == "1" {
                  fileupdate = true;
              } else {
                  if itemallvec[0] != "0" {
                      errstring = format!("Error line number {}: update not 0 or 1: {}", linenum, itemallvec[0]);
                      errcode = 19;
                      break;
                  }
              }
              let fileassxstr;
              let fileasssav =  itemallvec[4].trim().to_string();
              let filenamex = itemallvec[1].trim();
              let filelocsave = itemallvec[2].trim();
              let mut dateyrd = 0;
              let mut datemod = 0;
              let mut datedayd = 0;
              let mut datehrd = 0;
              let mut datemind = 0;
              let mut datesecd = 0;
              let mut datenumd = 0;
              if dateinname_bool {
// date in name start
                  let mut baddate1 = 0;
                  if filenamex.len() < 25 {
                      baddate1 = 1;
                  } else {
                      let date1ar2: Vec<&str> = filenamex[0..23].split("_").collect();
                      let lendat2 = date1ar2.len();
                      for indl in 0..lendat2 {
                           let date_int: i32 = date1ar2[indl].parse().unwrap_or(-9999);
                           if date_int == -9999 {
                               baddate1 = 1;
                               break;
                           } else {
                               match indl {
                                  0 => dateyrd = date_int,
                                  1 => datemod = date_int as u32,
                                  2 => datedayd = date_int as u32,
                                  3 => datehrd = date_int as i32,
                                  4 => datemind = date_int as i32,
                                  5 => datesecd = date_int as i32,
                                  6 => datenumd = date_int as i32,
                                  _ => baddate1 = 1,
                               }
                           }
                      }
                      if baddate1 == 0 {
                          let datexx = Local.with_ymd_and_hms(dateyrd, datemod, datedayd, 0, 0, 0);
                          if datexx == LocalResult::None {
                              baddate1 = 1;
                          } else {
                              if (datenumd < 0) | (datenumd > 999) {
                                  baddate1 = 1;
                              } else if (datehrd < 0) | (datehrd > 23) {
                                  baddate1 = 1;
                              } else if (datemind < 0) | (datemind > 59) {
                                  baddate1 = 1;
                              } else if (datesecd < 0) | (datesecd > 59) {
                                  baddate1 = 1;
                              }
                          }
                      }
                  }
                  if baddate1 != 0 {
// date in name end
                      errstring = format!("Error line number {}: invalid date in name for filename {}", linenum, filenamex);
                      errcode = 21;
                      break;
                  }
              }
              let filecurx = itemallvec[3].trim();
              let mut dateyr = 0;
              let mut datemo = 0;
              let mut dateday = 0;
              let mut datehr = 0;
              let mut datemin = 0;
              let mut datesec = 0;
              let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
              let after = re.replace_all(&filecurx, "_");
              let listdatex: Vec<&str> = after.split("_").collect();
              let lendat2 = listdatex.len();
              let mut baddate1 = 0;
              for indl in 0..lendat2 {
                   let date_int: i32 = listdatex[indl].parse().unwrap_or(-9999);
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
                           _ => baddate1 = 1,
                       }
                   }
              }
              let datexx = Local.with_ymd_and_hms(dateyr, datemo, dateday,1,1,1);
              if datexx == LocalResult::None {
                  baddate1 = 1;
              } else {
                  if (datehr < 0) | (datehr > 23) {
                      baddate1 = 1;
                  } else if (datemin < 0) | (datemin > 59) {
                      baddate1 = 1;
                  } else if (datesec < 0) | (datesec > 59) {
                      baddate1 = 1;
                  }
              }
              if baddate1 != 0 {
                  errstring = format!("Error line number {}: bad current date {}", linenum, filecurx);
                  errcode = 22;
                  break;
              }
              if dateinname_bool {
                  dateyr = dateyrd;
                  datemo = datemod;
                  dateday = datedayd;
                  datehr = datehrd;
                  datemin = datemind;
                  datesec = datesecd;
              } else {
                  if year_set {
                      dateyr = year_int;
                  }
                  if month_set {
                      datemo = month_int as u32;
                  }
                  if day_set {
                      dateday = day_int as u32;
                  }
                  let datexx = Local.with_ymd_and_hms(dateyr, datemo, dateday,1,1,1);
                  if datexx == LocalResult::None {
                      errstring = format!("Error line number {}: bad date for source or target {} {} {}", linenum, year_int, month_int, day_int);
                      errcode = 23;
                      break;
                  }
              }
              let mut dateto = Utc.with_ymd_and_hms(dateyr, datemo, dateday, datehr as u32, datemin as u32, datesec as u32).unwrap();
              if offsettest {
                  dateto = dateto + Duration::days(dateyr1*365) +
                                    Duration::days(datemo1*30) +
                                    Duration::days(dateday1) +
                                    Duration::hours(datehr1) +
                                    Duration::minutes(datemin1) +
                                    Duration::seconds(datesec1);
              }
              fileassxstr = format!("{}", dateto.format("%Y:%m:%d %H:%M:%S"));
              let mut fileassx = &fileassxstr;
              let mut filelocx = "Date Chg";
              let mut filenewdir = format!("{}", dateto.format("pic%Y/pic%Y%m%d"));
              if bolok {
                  if !all_bool {
                      if !fileupdate {
                          filelocx = filelocsave;
                          fileassx =  &fileasssav;
                          filenewdir = itemallvec[5].trim().to_string();
                      } 
                  }
                  newlist.push(format!("{}|{}|{}|{}|{}", filenamex, filelocx, filecurx, fileassx, filenewdir));
              }
//              println!("line num {} fileupdate -{}-  filenamex -{}-  filelocx -{}-  filecurx -{}-  fileassx -{}-  filenamex -{}-", 
//                         linenum, fileupdate, filenamex, filelocx, filecurx, fileassx, filenewnamex);
         }
     } 
     (errcode, errstring, newlist)
}

