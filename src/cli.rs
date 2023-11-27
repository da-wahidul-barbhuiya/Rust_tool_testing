use std::fmt::Display;
use std::{time::Duration, fs::File};
use std::collections::HashMap;
use clap::{Arg};
use std::io::{BufReader, BufRead, Lines};
use clap::Parser;
use time::PrimitiveDateTime;
use needletail::{parse_fastx_file, Sequence, FastxReader};
use std::error::Error;
use regex::Regex;
use NextLineTest::barcode_extraction;
use std::path::{Path, PathBuf};

fn check_path_exists(s: &str) -> Result<PathBuf, String> {
    // Implement your logic to check path existence here
    // ...

    // For the sake of the example, we assume s is a valid path
    Ok(PathBuf::from(s))
}

pub trait Summary {
    fn last_time(&self,start:PrimitiveDateTime)->PrimitiveDateTime;
    fn read_file(&self) -> Box<dyn FastxReader>;
    // fn barcode_time<'a,'b>(&'b self,barcode_map:&'a mut HashMap<String,Vec<i32>>)-> &'a mut HashMap<String,Vec<i32>>;
} 

#[derive(Parser,Debug)]
pub struct Arguments<P>
where
    P:AsRef<PathBuf>+std::clone::Clone+std::marker::Send+std::marker::Sync,
{
    pub from:u64,
    pub to:u64,
    #[clap(value_parser = check_path_exists, value_name = "FILE")]
    pub file_path:P,
}



impl <P>  Summary for Arguments<P>
where 
    P:AsRef<PathBuf>+std::clone::Clone+std::marker::Sync+std::marker::Send+ std::convert::AsRef<std::path::Path>,
{
    fn last_time(&self,start:PrimitiveDateTime)->PrimitiveDateTime {
        start+Duration::new(self.from*3600, 0)
    }
    fn read_file(&self) -> Box<dyn FastxReader>{
        let reader=parse_fastx_file(self.file_path).expect("Invalid file type");
        // let mut lines=reader.lines();
        reader
        
    }



    /*
    fn barcode_time<'a,'b>(&'b self,barcode_map:&'a mut HashMap<String,Vec<i32>>)-> &'a mut HashMap<String,Vec<i32>>{
        let reader=parse_fastx_file(&self.file_path).expect("Invalid file type");
        let mut lines=reader.lines();
        let date_time_re: Regex = Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
        let mut smallest_datetime:Option<PrimitiveDateTime>=None;
        while let Some(line) = lines.next() {
            let header = line.unwrap();

            if let Some(captures) = date_time_re.captures(&header) {
                let datetime_str = captures.name("time").unwrap().as_str();
                let sliced_datetime = &datetime_str[..19];
        
                if let  Ok(parsed_datetime)= PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S") {
                    match smallest_datetime {
                        Some(smallest)=>{
                            if parsed_datetime <smallest  {
                                smallest_datetime=Some(parsed_datetime);
                                
                            }
                            if let Some(smallest) =  smallest_datetime{
                                let new_added_time=self.last_time(smallest);
                                if parsed_datetime<new_added_time{
                                    // println!("Parsed date time frame from match expression:{}",parsed_datetime);
                                    let barcode_no=barcode_extraction(header);
                                    // println!("Barcode name:{}",barcode_no);
                                    
                                    if let Some(sequence_line) = lines.next() {
                                        // println!("Sequence line is:{:?}",sequence_line);
                                        if let Ok(seq_len)=sequence_line{
                                            let line_length:i32=seq_len.len().try_into().unwrap();
                                            // println!("Sequence line length is :{}",line_length);
                                            // barcode_reads_length.insert(barcode_no, line_length);
                                            barcode_map.entry(barcode_no).and_modify(|vec|vec.push(line_length)).or_insert(vec![line_length]);
                                            

                                        }
                                        
                                    }
                                    
                                }
                            }
                        }
                        None=>{
                            smallest_datetime=Some(parsed_datetime);
                        }
                    }
                }
            }
           
        }
        // println!("barcode with read length:{:?}",barcode_map);
        barcode_map


    }*/
    
}





//-----------------------------------------------------------\\
/*                                                        
|    input command : cargo run from_time to_time file_name    |
|                    cargo run 1 4 diff_barcode.fastq         |
|    Output should look like this                             |
|    MRL summary                                              |
|    Time barcode05 barcode04                                 |
|    1hr   0         680                                      |
|    2hr   896       693.5                                    |
|    3hr   667       606                                      |
|    4hr   667       606                                      |
|                                                             |
*/
//------------------------------------------------------------\\    