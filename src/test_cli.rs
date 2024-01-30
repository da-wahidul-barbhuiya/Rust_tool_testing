use std::{collections::HashMap, ffi::OsStr, fmt::format, fs::read, io::BufRead, path::{Path, PathBuf, self}, str::from_utf8, time::Duration, vec};
use lazy_static::lazy_static;
use clap::Parser;
use csv::Reader;
use regex::Regex;
use time::PrimitiveDateTime;
use needletail::{parse_fastx_file, parser::SequenceRecord, FastxReader, Sequence};
// #[derive(Parser,Debug)]
// #[command(author,version,about,long_about=None)]

// pub struct Cli{
//     //input fastq file
//     #[clap(value_parser=check_path_exists,value_name="FILE")]
//     pub input:PathBuf,
//     // summary output mentioning last hour
//     #[clap(short='h',long="hour",value parser="summary_time",value_name="DURATION",allow_hyphen_values=true)]
//     pub last_hr:PrimitiveDateTime
// }


// // checking path exists or not
// fn check_path_exists<S:AsRef<OsStr>+?sized>(s:&S) ->Result<PuthBuf,String>{
//     let path=PathBuf::from(s);
//     if path.exists(){
//         Ok(path)
//     }
//     else{
//         Err(format!("{:?} does not exist",path))
//     }

// }
// fn summary_time(start:PrimitiveDateTime)->PrimitiveDateTime{
//     start+Duration::new()

// }

// testing trait
// pub trait BarcodeRead {
//     fn barcode_reads_count<'a,'b>(&'b self,barcode_map:&'a mut HashMap<String,Vec<i32>>)-> &'a mut HashMap<String,Vec<i32>>;
    
// }

#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
pub struct Argument{
    // file name that should be provided
    #[arg(value_name="FILE",value_parser=checking_file_path)]
    pub file:PathBuf,
    #[arg(short,long)]
    pub summary:String
}

pub fn checking_file_path(file_name:&str)-> Result<std::path::PathBuf,String>{
    let path=std::path::PathBuf::from(file_name);
    if path.exists(){
        Ok(path)
    } else {
        Err(format!("File not found {}",file_name))
    }

}
// function to calculate one hour time interval
pub fn one_hr_time(file_arg:Argument)-> PrimitiveDateTime{
    let time_to_pass=barcode_count(file_arg).unwrap();
    time_to_pass+Duration::new(3600, 0)
}
// update smallest  datetimeframe to  one hour interval
// try to find how recursive function is used in rust
// you can use match enum for passing same function and add one argument over there in the struct Argument
// pub fn hour_repetation(parsed_arg:Argument)->PrimitiveDateTime{
//     let new_dtime=one_hr_time(parsed_arg);
//     match new_dtime {
//         arg=>{
//             let post_dtime=one_hr_time(parsed_arg);
//             post_dtime
//         }
//     }
// }
lazy_static!{
    static ref barcode_re:Regex=Regex::new(r"barcode=(?P<barcode>\S+)\s*").unwrap();
    static ref datetime_re:Regex=Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
}
pub fn barcode_reads(lin_str:String)->String{
    if let Some(cap_barcode) = barcode_re.captures(&lin_str) {
        let barcode_str=cap_barcode.name("barcode").unwrap().as_str();
        return barcode_str.to_string();
    }
    String::new()
}

// this function will give a hashmap having different barcode name and it's read length 
impl Argument {
    pub fn barcode_reads_count<'a,'b>(&'b self,barcode_map:&'a mut HashMap<String,Vec<i32>>)-> &'a mut HashMap<String,Vec<i32>>
    {
    let file_read=&self.file.clone().into_os_string().into_string().unwrap();
    let mut reader=parse_fastx_file(file_read).expect("Invalid file/path");
    let mut smallest_dtime_frame:Option<PrimitiveDateTime>=None;
    while let Some(record) =reader.next()  {
        let seqreq=record.expect("Invalid records!");
        let header=seqreq.id().lines().next().unwrap().unwrap();
        if let Some(time_captures) = datetime_re.captures(&header) {
            let datetime_str=time_captures.name("time").unwrap().as_str();
            let sliced_dtime=&datetime_str[..19];
            if let Ok(parsed_dt) = PrimitiveDateTime::parse(sliced_dtime, "%Y-%m-%dT%H:%M:%S") {
                match smallest_dtime_frame {
                    Some(smallest)=>{
                        if parsed_dt<smallest{
                            smallest_dtime_frame=Some(parsed_dt)
                        }
                        if let Some(smallest) = smallest_dtime_frame {
                            let one_hr_time=smallest+Duration::new(3600, 0);
                            if parsed_dt<one_hr_time{
                                let barcode_no=barcode_reads(header);
                                let seq_line: i32=seqreq.seq().lines().next().unwrap().unwrap().len() as i32;
                                barcode_map.entry(barcode_no).and_modify(|vec|vec.push(seq_line)).or_insert(vec![seq_line]);
                            }
                            
                            
                        }
                    }
                    None=>{
                        smallest_dtime_frame=Some(parsed_dt)
                    }
                    
                }
                
            }
            
        }
        
    }
    barcode_map
}
    
}

// function/method for finding smallest time
// use a different crate for parsing fastq file
// first try to implement a single hour and its barcode with reads number 
// then implement a for loop so that it can iterate the process untill the end
// impl Argument{
pub fn barcode_count(arg:Argument)-> Option<PrimitiveDateTime>{
    let fastq_file=arg.file.into_os_string().into_string().unwrap();
    let mut reader=parse_fastx_file(fastq_file).expect("Valid file/path");
    let mut smallest_time: Option<PrimitiveDateTime>=None;
    let mut smallest_dtime:Option<PrimitiveDateTime>=None;
    while let Some(record) = reader.next() {
        let mut seqreq=record.expect("invalid records");
        let p=seqreq.id().lines();
        let seqreq_str=seqreq.id().lines().next().unwrap().unwrap();
        if let Some(captures)=datetime_re.captures(&seqreq_str){
            let datetime_str=captures.name("time").unwrap().as_str();
            let sliced_datetime=&datetime_str[..19];
            if let Ok(parsed_dt) = PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S") {
                match smallest_dtime {
                    Some(smallest)=>{
                        if parsed_dt<smallest{
                            smallest_dtime=Some(parsed_dt);
                        }
                    }
                    None=>{
                        smallest_dtime=Some(parsed_dt);
                    }
                    
                }
                
            }
        }
        // let time_frame:String=seqreq.id().iter().map(|&a|a as char);
        /*
        let time_search=seqreq.id().lines().map(|x|x.find(|line|line.contains("start_time=")));
        let first_try=seqreq.id().as_ref().map(|x| x.lines().find(|line| line.contains("start_time=")));
        let k=seqreq.id().into_iter().next().as_deref().filter(|x| x:from_utf8(&str));
        
        let second_try=seqreq.id().lines().find(|x| x.contains("Ss").unwrap());
        // .binary_search("start_time=");
        seqreq.iterator().map(|x:String|x.to_string());
        // let read_count=seqreq.seq().lines().count();
        // converting header from ASCII to readable string
        // let line_str=record.unwrap();
        // date_time_re.captures(seqreq).and_then(|dt_capture|{
        //     dt_capture.name("time").map(|time|time.as_str())
        // });
        datetime_re.find_iter(seqreq).map(|dt|dt.as_str()).collect();
        let line=record.into_iter().map(|a:String|a.to_lowercase());
        if let Some(captures) = date_time_re.captures(&line_str) {
            let datetime_str=captures.name("time").unwrap().as_str();
            let sliced_datetime=&datetime_str[..19];
            let k=seqreq.id().lines();
        } */
    }
    smallest_dtime
}
// }
