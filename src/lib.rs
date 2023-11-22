use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, BufRead, Lines};
use std::thread::Builder;
use std::{vec, usize, path, primitive};
use regex::Regex;
use std::cmp::Ordering;
use csv;
use std::{fs::File, time::Duration};
use time::{PrimitiveDateTime, date,time};


pub trait FastqFileRead {
    fn end_time(&self,start:PrimitiveDateTime)->PrimitiveDateTime;
    fn count_line(self, collect_map:&mut HashMap<String,Vec<i32>>)->&mut HashMap<String, Vec<i32>>;
    fn reframe(&self)-> PrimitiveDateTime;
    fn start_time( &self)->PrimitiveDateTime;
    fn get_line( &self)->PrimitiveDateTime;
    fn in_btn_time<'a,'b>(&'b self,barcode_map:&'a mut HashMap<String,Vec<i32>>)->&'a mut HashMap<String,Vec<i32>>;
}


// creating struct for clr argument passing
pub struct Config{
    pub time_hr:u64,
    pub file_name:File,
}

// //passing trait inside a field of another struct
// pub struct TimeFrame{
//     small:Box<dyn FastqFileRead>,
//     large:Box<dyn FastqFileRead>,
// }
// impl TimeFrame {
//     fn get_small(self)-> Box<dyn FastqFileRead>{
//         let small=self.small;
//         small.start_time()
//     }
    
// }
//creating a struct and its match expression for barcode and time extraction using regular expression
/*
struct ExtractionProcess{
    time:PrimitiveDateTime,
    barcode:String
}
fn something(config:Config){
    file
    match output{
        config.time=>{ 
            let date_time_re: Regex = Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();

        }
        config.barcode=>{
            let barcode_re=Regex::new(r"barcode=(?P<barcode>\S+)\s*").unwrap();
        }
    }

}





*/
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
impl FastqFileRead for Config {
    fn end_time(&self,start:PrimitiveDateTime) ->PrimitiveDateTime{
        start+Duration::new(self.time_hr*3600, 0)
    }
    fn count_line(self, collect_map:&mut HashMap<String,Vec<i32>>)-> &mut HashMap<String, Vec<i32>>{
        let reader=BufReader::new(self.file_name);
        let mut lines=reader.lines();
        while let Some(line) =lines.next()  {
            let line_str=line.unwrap();
            // println!("Header line:{}",line_str);
        
            if let Some(next_line) =lines.next()  {
                
                // println!("sequence line:{:?}",next_line);
                if let Ok(next_line_result) =next_line  {
                    let line_length: i32=next_line_result.len().try_into().unwrap();
                    println!("Sequence line length:{:?}",line_length);
                    // let line_itr=lines.next().unwrap();
                    // let lin_str=line_itr.unwrap();
    
                    let barcode_name=barcode_extraction(line_str);
                    // let barcode_name_cl=barcode_name.clone();
                    collect_map.entry(barcode_name).and_modify(|vec| vec.push(line_length)).or_insert(vec![line_length]);
                    
                }else if let Err(err)=next_line {
                    eprintln!("Error reading line:{:?}",err);
                    
                };
                
            }
            if let Some(after_next_line) =lines.next()  {
                // println!("Plus line:{:?}",after_next_line);
            }
            if let Some(fourth_line) =lines.next()  {
                // println!("QC line:{:?}",fourth_line);
            }
        }
        collect_map
        
    }
    //reframing structure
    fn reframe(&self)-> PrimitiveDateTime{
        let reader=BufReader::new(&self.file_name);
        let mut lines=reader.lines();
        let mut smallest_datetime:Option<PrimitiveDateTime>=None;
        let date_time_re: Regex = Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
        // let mut end_time:PrimitiveDateTime = Default::default();
        while let Some(line)=lines.next() {
            let line_str=line.unwrap();
            if let Some(captures) = date_time_re.captures(&line_str) {
                let datetime_str = captures.name("time").unwrap().as_str();
                let sliced_datetime=&datetime_str[..19];
                println!("Sliced time{}",sliced_datetime);
                let parsed_time=PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S");
                // return parsed_time;
                match parsed_time {
                    Ok(parsed_time)=>{
                        // println!("Each parsed date time :{}",parsed_time);
                        return parsed_time;
                    },
                    Err(_)=>{println!("Error parsing date time frame");}
                    
                }
                // if let  Ok(parsed_datetime)= PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S") {
                //     return parsed_datetime;
                // }
            }
            
        }
        // loop{
        //     if let Some(line) =lines.next()  {
        //         let line_str=line.unwrap();
        //         if let Some(captures) =date_time_re.captures(&line_str)  {
        //             let datetime_str = captures.name("time").unwrap().as_str();
        //                     let sliced_datetime=&datetime_str[..19];
        //                     println!("Sliced date time:{}",sliced_datetime);
        //                     // if let  Ok(parsed_datetime)= PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S") {
        //                     //     return parsed_datetime;
            
        //                     // }  
                    
        //         }
                
        //     }
        // }
        PrimitiveDateTime::new(date!(1930-01-01), time!(0:00)) //this is dummy value ; replace this with some error handling stuff
    }

    fn start_time( &self)->PrimitiveDateTime {
        // let reader=BufReader::new(&self.file_name);
        // let mut lines=reader.lines();
        let mut smallest_datetime:Option<PrimitiveDateTime>=None;
        let mut small=String::new();
        // let date_time_re: Regex = Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
        // // let mut end_time:PrimitiveDateTime = Default::default();
        // while let Some(line)=lines.next() {
        //     let line_str=line.unwrap();
        //     if let Some(captures) = date_time_re.captures(&line_str) {
        //         let datetime_str = captures.name("time").unwrap().as_str();
        //         let sliced_datetime=&datetime_str[..19];
        //         if let  Ok(parsed_datetime)= PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S") {
            let  parsed_time=&self.reframe();
            println!("parsed time:{}",parsed_time);
            match smallest_datetime {
                Some(smallest)=>{
                    if parsed_time <&smallest  {
                        smallest_datetime=Some(*parsed_time);
                        return  smallest_datetime.unwrap();
                    }
                }
                None=>{
                    smallest_datetime=Some(*parsed_time);
                }
            }
        //         }
        //     }
        // }
        PrimitiveDateTime::new(date!(1930-01-01), time!(0:00)) //this is dummy value ; replace this with some error handling stuff
    }
    fn get_line( &self)->PrimitiveDateTime {
        let reader=BufReader::new(&self.file_name);
        let mut lines=reader.lines();
        let mut smallest_datetime:Option<PrimitiveDateTime>=None;
        let date_time_re: Regex = Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
        // let mut end_time:PrimitiveDateTime = Default::default();
        while let Some(line)=lines.next() {
            let line_str=line.unwrap();
            if let Some(captures) = date_time_re.captures(&line_str) {
                let datetime_str = captures.name("time").unwrap().as_str();
                let sliced_datetime=&datetime_str[..19];
                if let  Ok(parsed_datetime)= PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S") {
                    match smallest_datetime {
                        Some(smallest)=>{
                            if parsed_datetime <smallest  {
                                smallest_datetime=Some(parsed_datetime);
                                if let Some(start_time) = smallest_datetime {
                                    let end_time=self.end_time(start_time);
                                    return end_time;
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
        PrimitiveDateTime::new(date!(1930-01-01), time!(0:00)) //this is dummy value ; replace this with some error handling stuff
    }
    fn in_btn_time<'a,'b>(&'b self,barcode_map:&'a mut HashMap<String,Vec<i32>>)-> &'a mut HashMap<String,Vec<i32>>{
        let reader=BufReader::new(&self.file_name);
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
                                let new_added_time=self.end_time(smallest);
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


    }
    
}
pub fn time_extraction(mut lines:Lines<BufReader<File>>)-> Option<PrimitiveDateTime>{
    let date_time_re:Regex=Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
    let mut smallest_datetime:Option<PrimitiveDateTime>=None;
    while let Some(line)=lines.next() {
        let line_str=line.unwrap();
        if let Some(captures)=date_time_re.captures(&line_str){
            let datetime_str=captures.name("time").unwrap().as_str();
            let sliced_datetime=&datetime_str[..19];
            if let Ok(parsed_datetime)=PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S"){
                // if let Some(smallest)=smallest_datetime{
                //     if parsed_datetime<smallest{
                //         smallest_datetime=Some(parsed_datetime);
                //     }else {
                //         smallest_datetime=Some(parsed_datetime);
                //     }
                // }
                match smallest_datetime {
                    Some(smallest)=>{
                        if parsed_datetime<smallest{
                            smallest_datetime=Some(parsed_datetime);
                        }
                    }
                    None=>{
                        smallest_datetime=Some(parsed_datetime);
                    }
                    
                }
            }
        }
        
    }
    
    
    smallest_datetime

}
    pub fn barcode_extraction(lin_str:String)-> String{
        let barcode_re=Regex::new(r"barcode=(?P<barcode>\S+)\s*").unwrap();
        if let Some(cap_barcode) = barcode_re.captures(&lin_str) {
            let barcode_str=cap_barcode.name("barcode").unwrap().as_str();
            // println!("barcode string part:{}",barcode_str);
            return  barcode_str.to_string();
            
        }
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
    fn time_extract_test(){
        let text="@reads1 start_time=2023-06-01T12:47:06.339862+05:30\nA\n+\n@";
        let mut file=Builder::new().suffix(".fastq").tempfile().unwrap();
        file.write_all(text.as_bytes()).unwrap();

        // let mut reader=parse_fastx_file(file.path()).unwrap();

        // let rec=reader.next().unwrap();
        // let records=rec.unwrap();
        let test_file=File::open(file.path()).unwrap();
        let reader=BufReader::new(test_file);
        let mut lines=reader.lines();
        let line_res=lines.next();
        if let Some(line_st)=line_res{
            let line_str=line_st.unwrap();
            let date_time_test:Regex=Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
            if let Some(captures) =date_time_test.captures(&line_str)  {
                let datetime_test_str=captures.name("time").unwrap().as_str();
                let sliced_datetime_test=&datetime_test_str[..19];
                // let expected_time=PrimitiveDateTime::new(datetime!());
                assert_eq!(sliced_datetime_test,"2023-06-01T12:47:06")
                
            }
        }
       
    }
    #[test]
    fn added_time_test(){
        let text="@reads1 start_time=2023-06-01T12:47:06.339862+05:30\nA\n+\n@";
        let mut file=Builder::new().suffix(".fastq").tempfile().unwrap();
        file.write_all(text.as_bytes()).unwrap();
        let test_file=File::open(file.path()).unwrap();
        let config_test=Config{time_hr:3,file_name:test_file};
        let actual=config_test.end_time(PrimitiveDateTime::new(date!(2023-06-01), time!(12:47:06)));
        let expected_time=PrimitiveDateTime::new(date!(2023-06-01), time!(15:47:06));
        assert_eq!(actual,expected_time)
    }
    // testing for smallest time frame from the file
    #[test]
    fn smallest_time_test(){
        let text="@reads1 start_time=2023-06-01T12:47:06.339862+05:30\nA\n+\n@\n@reads2 start_time=2023-06-01T13:56:04.339862+05:30\nT\n+\n#";
        let mut file=Builder::new().suffix(".fastq").tempfile().unwrap();
        file.write_all(text.as_bytes()).unwrap();
        let test_file=File::open(file.path()).unwrap();
        let rec=BufReader::new(test_file);
        let line=rec.lines();
        let actual_time=time_extraction(line);
        let expected_time=Some(PrimitiveDateTime::new(date!(2023-06-01), time!(12:47:06)));
        
        assert_eq!(actual_time,expected_time)
    }
    #[test]
    fn barcode_extract_test(){
        let text="@reads1 start_time=2023-06-01T12:47:06.339862+05:30 barcode=barcode01\nA\n+\n@\n@reads2 start_time=2023-06-01T13:56:04.339862+05:30 barcode=barcode02\nT\n+\n#";
        let mut file=Builder::new().suffix(".fastq").tempfile().unwrap();
        file.write_all(text.as_bytes()).unwrap();
        let test_file=File::open(file.path()).unwrap();
        let mut barcode_map:HashMap<String,Vec<i32>>=HashMap::new(); 
        let config_test=Config{time_hr:3,file_name:test_file};
        let mut expected_map:HashMap<String,Vec<i32>>=HashMap::new(); 
        expected_map.insert(String::from("barcode01"), [1].to_vec());
        expected_map.insert(String::from("barcode02"), [1].to_vec());
        let actual=config_test.count_line(&mut barcode_map);
        assert_eq!(actual,&mut expected_map);
        
        
    }
}
