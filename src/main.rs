use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Duration;
use std::{vec, usize};
use regex::Regex;
use time::PrimitiveDateTime;
use std::cmp::Ordering;
fn after(start:PrimitiveDateTime,time:&u64) ->PrimitiveDateTime{
    start+Duration::new(time*3600, 0)
}
fn time_iter(&mut input_vec:Vec<String>,start:Option<PrimitiveDateTime>, end:PrimitiveDateTime){
        // let mut datetimecout:Vec<String>=Vec::new();
        if let Some(smallest_time)=start{
            //println!("Smallest time is from the function :{}",smallest_time);
            if end > smallest_time{
                //println!("Parsed time from the function :{:?}",end);
                input_vec.push(end.to_string());
                println!("Datetime vector:{:?}",input_vec);
            }
            
        }
        else {
            println!("Not match for this case")
        }
    }

// fn get_all_timestamp(file:BufReader<File>,start:PrimitiveDateTime,current:PrimitiveDateTime,end:PrimitiveDateTime){
//     let adding_time:Vec<String>=Vec::new();
//     for line in file.lines(){
//         if start<current && current< end{

//         }
        
//     }
// }
// fn merge_sort(left:PrimitiveDateTime,right:PrimitiveDateTime)->(Vec<bool>,usize){
    

// }
// fn valid_indices(timestamp:&[PrimitiveDateTime],earliest:&PrimitiveDateTime,latest:&PrimitiveDateTime) -> (){
//     let mut to_keep:Vec<bool>=vec![false;timestamp.len()];
//     let mut nb_reads_to_keep=0;
//     timestamp.iter().enumerate().for_each(|(i,t)|{
//         if earliest<=t && t<=latest{
//             to_keep[i]=true;
//             nb_reads_to_keep+=1;
//         }
//         let (Vec,usize)=(&to_keep,nb_reads_to_keep);
//         ()
    

//     });
// }
//test from the github page


#[derive(Debug, Clone, Copy)]
struct AddTime<T>{
    value: T
}

impl<T:Copy+std::ops::AddAssign,R:Into<AddTime<T>>> std::ops::AddAssign<R>
for  AddTime<T>
{
   fn add_assign(&mut self, rhs: R) {
       let rhs=rhs.into();
       self.value+=rhs.value
   } 
}
fn main() -> std::io::Result<()> {
    let file = File::open("new.fastq")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut smallest_datetime: Option<PrimitiveDateTime> = None;
    //let mut smallest_datetime = AddTime { value: PrimitiveDateTime::now() };
    let date_time_re: Regex = Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
    let mut date_time_take:Vec<String>=Vec::new();
    while let Some(line) = lines.next() {
        let line_str = line?;
        if let Some(captures) = date_time_re.captures(&line_str) {
            let datetime_str = captures.name("time").unwrap().as_str();
            let sliced_datetime = &datetime_str[..19];
            if let Ok(parsed_datetime) = PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S") {
                // println!("Date time frame: {}", parsed_datetime);
                if let Some(smallest) = smallest_datetime {
                    if parsed_datetime < smallest {
                        smallest_datetime = Some(parsed_datetime);
                        // println!("Parsed time for every frame:{}",parsed_datetime);
                        println!("Header line is: {:?}", line_str);
                        if let Some(next_line) = lines.next() {
                            println!("Sequence line is: {:?}", next_line?);
                        }
                        if let Some(after_next_line) = lines.next() {
                            println!("Plus line: {:?}", after_next_line?);
                        }
                        if let Some(fourth_line) = lines.next() {
                            println!("QC line: {:?}", fourth_line?);
                        }
                        // smallest+=parsed_datetime
                        
                    }
                } else {
                    smallest_datetime = Some(parsed_datetime);
                }
                time_iter(date_time_take,smallest_datetime,parsed_datetime);
                // valid_indices(smallest_datetime, &parsed_datetime, &parsed_datetime)
            }
        }
        
    }

    if let Some(smallest) = smallest_datetime {
        let added_time=after(smallest,&3);
        println!("Smallest datetime frame: {}", smallest);
        println!("New added time is :{}",added_time);
        
    } else {
        println!("No datetime frames found in the file.");
    }

    Ok(())
}
