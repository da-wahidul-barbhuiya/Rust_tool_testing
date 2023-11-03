use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, BufRead, Lines};
use std::{vec, usize, path, primitive};
use regex::Regex;
use std::cmp::Ordering;
use csv;

use std::{fs::File, time::Duration};
use time::PrimitiveDateTime;



pub trait FastqTimeRead {
    fn end_time(time:&Self,start:PrimitiveDateTime)->PrimitiveDateTime;
    // fn 
}


// creating struct for clr argument passing
pub struct Config{
    pub time_hr:u64,
    pub file_name:File,
}
impl Config{
    pub fn build<a>(args:&[a]) ->Result<Config,&'static str>
    where a:AsRef<str>{
        if args.len()<3{
            return Err("Not enough arguments");
        }
        // converting time_hr to String type 
        let time_hr=args[1].as_ref().to_owned();
        // converting time_hr from String to i64 type
        let time_hr_i64=time_hr.trim().parse().map_err(|_| "Failed to parse time into int type")?;

        // converting file_name to String type
        let file_name=args[2].as_ref();
        let file=File::open(file_name).map_err(|_|"Failed to open file")?;
        Ok(Config { time_hr: time_hr_i64, file_name: file })
    }
}
impl FastqTimeRead for Config {
    fn end_time(time:&Self,start:PrimitiveDateTime) ->PrimitiveDateTime{
        start+Duration::new(time.time_hr*3600, 0)
    }
}

pub fn line_count(ln_str:String,  line:&mut std::io::Lines<BufReader<File>>, collect_map:&mut HashMap<String,Vec<i32>>){
    // println!("Header line:{:?}",ln_str);
    if let Some(next_line) =line.next()  {
        
        // println!("sequence line:{:?}",next_line);
        if let Ok(next_line_result) =next_line  {
            let line_length: i32=next_line_result.len().try_into().unwrap();
            // println!("Sequence line length:{:?}",line_length);
            let barcode_name=barcode_extraction(ln_str);
            collect_map.entry(barcode_name).and_modify(|vec| vec.push(line_length)).or_insert(vec![line_length]);
            
        }else if let Err(err)=next_line {
            eprintln!("Error reading line:{:?}",err);
            
        };
        
    }
    if let Some(after_next_line) =line.next()  {
        // println!("Plus line:{:?}",after_next_line);
    }
    if let Some(fourth_line) =line.next()  {
        // println!("QC line:{:?}",fourth_line);
    }
}

pub fn barcode_extraction(lin_str:String)->//std::io::Result<()>{
    String{
    let barcode_re=Regex::new(r"barcode=(?P<barcode>\S+)\s*").unwrap();
    if let Some(cap_barcode) = barcode_re.captures(&lin_str) {
        let barcode_str=cap_barcode.name("barcode").unwrap().as_str();
        // println!("barcode string part:{}",barcode_str);
        return  barcode_str.to_string();
        
    }
    // Ok(())
    String::new()
}

#[cfg(test)]
mod test{
    use super::*;
    use time::{time,date};
    use std::io::Write;
    use tempfile::Builder;
    use needletail::parse_fastx_file;
    #[test]
    fn end_time_test(){
        let start_time=PrimitiveDateTime::new(date!(2023-06-01), time!(12:47:06));
        let added_time=start_time+Duration::new(3*3600, 0);
        let expected_time=PrimitiveDateTime::new(date!(2023-06-01), time!(15:47:06));
        assert_eq!(added_time,expected_time)
    }
    fn end_time_fastq_file_test(){
        let text="@reads1 st:Z:2023-06-01 12:47:06.356+00:00\nA\n+\n@";
        let mut file=Builder::new().suffix(".fastq").tempfile().unwrap();
        file.write_all(text.as_bytes()).unwrap();

        let mut reader=parse_fastx_file(file.path()).unwrap();
        let rec=reader.next().unwrap();
        let records=rec.unwrap();
        // let actual=records.end_time().unwrap();
        let config_test=Config{time_hr:3,file_name:reader};
        let actual=records.end_time(config_test,PrimitiveDateTime::new(date!(2023-06-01), time!(12:47:06)));
        
        let expected_time=PrimitiveDateTime::new(date!(2023-06-01), time!(15:47:06));
        assert_eq!(actual,expected_time)

    }
}