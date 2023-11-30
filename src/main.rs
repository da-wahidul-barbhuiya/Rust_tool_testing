// use std::collections::HashMap;
// use std::error::Error;
// use std::fs::File;
// use std::io::{BufReader, BufRead, Lines};
// use std::time::Duration;
// use std::{vec, usize, path, primitive};
// use regex::Regex;
// use time::PrimitiveDateTime;
// use std::cmp::Ordering;
// use csv;


// fn after(start:PrimitiveDateTime,time:&u64) ->PrimitiveDateTime{
//     start+Duration::new(time*3600, 0)
// }

// fn line_count(ln_str:String,  line:&mut std::io::Lines<BufReader<File>>, collect_map:&mut HashMap<String,Vec<i32>>){
//     // println!("Header line:{:?}",ln_str);
//     if let Some(next_line) =line.next()  {
        
//         // println!("sequence line:{:?}",next_line);
//         if let Ok(next_line_result) =next_line  {
//             let line_length: i32=next_line_result.len().try_into().unwrap();
//             // println!("Sequence line length:{:?}",line_length);
//             let barcode_name=barcode_extraction(ln_str);
//             collect_map.entry(barcode_name).and_modify(|vec| vec.push(line_length)).or_insert(vec![line_length]);
            
//         }else if let Err(err)=next_line {
//             eprintln!("Error reading line:{:?}",err);
            
//         };
        
//     }
//     if let Some(after_next_line) =line.next()  {
//         // println!("Plus line:{:?}",after_next_line);
//     }
//     if let Some(fourth_line) =line.next()  {
//         // println!("QC line:{:?}",fourth_line);
//     }
// }
// fn barcode_extraction(lin_str:String)->//std::io::Result<()>{
//     String{
//     let barcode_re=Regex::new(r"barcode=(?P<barcode>\S+)\s*").unwrap();
//     if let Some(cap_barcode) = barcode_re.captures(&lin_str) {
//         let barcode_str=cap_barcode.name("barcode").unwrap().as_str();
//         // println!("barcode string part:{}",barcode_str);
//         return  barcode_str.to_string();
        
//     }
//     // Ok(())
//     String::new()
// }

// fn main() -> std::io::Result<()> {
//     let file = File::open("diff_barcode.fastq")?;
//     let reader = BufReader::new(file);
//     let mut lines = reader.lines();
//     let mut smallest_datetime: Option<PrimitiveDateTime> = None;
//     //let mut smallest_datetime = AddTime { value: PrimitiveDateTime::now() };
//     let date_time_re: Regex = Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
//     let mut date_time_take:Vec<String>=Vec::new();
//     let mut Collect_barcode:HashMap<String,Vec<i32>>=HashMap::new();
//     while let Some(line) = lines.next() {
//         let line_str = line?;
//         if let Some(captures) = date_time_re.captures(&line_str) {
//             let datetime_str = captures.name("time").unwrap().as_str();
//             let sliced_datetime = &datetime_str[..19];
//             if let Ok(parsed_datetime) = PrimitiveDateTime::parse(sliced_datetime, "%Y-%m-%dT%H:%M:%S") {
//                 // println!("Date time frame: {}", parsed_datetime);
//                 //getting smallest date time
//                 if let Some(smallest) = smallest_datetime {
//                     if parsed_datetime < smallest {
//                         smallest_datetime = Some(parsed_datetime);
//                         // println!("Parsed time for every frame:{}",parsed_datetime);
//                         // println!("Smallesst date time frame ");
//                         // line_count(line_str.clone(), &mut lines,Collect_barcode);
//                         // println!("Header line is: {:?}", line_str);
//                         // if let Some(next_line) = lines.next() {
//                         //     println!("Sequence line is: {:?}", next_line?);
//                         // }
//                         // if let Some(after_next_line) = lines.next() {
//                         //     println!("Plus line: {:?}", after_next_line?);
//                         // }
//                         // if let Some(fourth_line) = lines.next() {
//                         //     println!("QC line: {:?}", fourth_line?);
//                         // }
//                         // smallest+=parsed_datetime
//                         // println!("smallest time inside the loop:{:?}",smallest_datetime);
//                         let poke_time=after(smallest, &3);
//                         // println!("Poke time inside the loop:{}",poke_time);
//                         // println!("Add struct inside the while loop:{:?}",point);
                        
//                     }
//                     //println!("Smallest time is outside the box:{:?}",smallest_datetime);
//                     date_time_take.push(parsed_datetime.to_string());
//                     // println!("Date time take:{:?}",date_time_take);

//                 } else {
//                     smallest_datetime = Some(parsed_datetime);
//                 }
//                 //new scope for comparing datetimeframe
//                 if let Some(new) =smallest_datetime  {
//                 let test_time=after(new, &7);
//                 if parsed_datetime< test_time{
//                     //println!("Parsed time within the range:{}",parsed_datetime);
//                     line_count(line_str.clone(),&mut lines,&mut Collect_barcode);
//                 }
                    
//                 }
//             }
//         }
        
//     }
//     // println!("Date time taking vector without sort:{:?}",date_time_take);
//     //println!("Collect barcode :{:?}",Collect_barcode);
//     let mut mean_map: HashMap<String, f64>=HashMap::new();
//     for (barcode_name,values) in Collect_barcode.iter(){
//         let barcode_mean:f64=values.iter().map(|&x|x as f64).sum::<f64>()/values.len() as f64;
//         mean_map.insert(barcode_name.clone(), barcode_mean);
//     }
//     println!("7 hours:{:#?}",mean_map);
    
//     // csv_writer(mean_map);

//     if let Some(smallest) = smallest_datetime {
//         let added_time=after(smallest,&3);
//         // println!("Smallest datetime frame: {}", smallest);
//         // println!("New added time is :{}",added_time);
        
//     } else {
//         println!("No datetime frames found in the file.");
//     }

//     Ok(())
// }
// // fn csv_writer(map_content:HashMap<String,f64>){
// //     // creating a csv handler
// //     let mut wtr=csv::Writer::from_path("Output.csv");
// //     let mut writer= match wtr{
// //         Ok(writer)=>writer,
// //         Err(err)=>return (),
// //     };
// //     for (keys,_) in &map_content{
// //         let new_keys: Vec<String> = map_content.keys().cloned().collect();
// //         let header=match writer.write_record(&[&keys]) {
// //             Ok(header)=> header,
// //             Err(err) => return (),
            
// //         };
// //     }
// //     // for (_,values) in &map_content{
// //     //     let new_values: Vec<&str> = map_content.values().map(|value| value.to_string().as_str()).collect();
// //     //     let records_value=match writer.write_record(&[&new_values]){
// //     //         Ok(records_value)=> records_value,
// //     //         Err(err)=> return (),
// //     //     };
// //     // }
// //     // writer.write_record(&["Time","Barcode"]);
// //     // for (key,value) in map_content{
// //     //     let record_result=match writer.write_record(&[
// //     //         "7th hour",
// //     //         &value.to_string(),
// //     //     ]){
// //     //         Ok(record_result) => record_result,
// //     //         Err(err) => return (),            
// //     //     };
// //     // }
// // }


use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process;
use NextLineTest::Config;
/* 
fn main() {
    let args:Vec<String>=env::args().collect();

    let  config=Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    println!("Time Stamp:{} hr",config.time_hr);
    println!("File name:{:?}",config.file_name);
    // if let Err(e)=NextLineTest::run(config){
    //     println!("Application error:{e}");
    //     process::exit(1);
    // }
    let mut Collect_barcode:HashMap<String,Vec<i32>>=HashMap::new(); 
    // let get_barcode=NextLineTest::FastqFileRead::count_line(config,  &mut Collect_barcode);
    // println!("Collecting all barocodes with their mrl:{:?}",get_barcode);
    // let Line_count=NextLineTest::FastqFileRead::get_line(&config);
    // println!("Added time:{:?}",Line_count)
    // let start_time=NextLineTest::FastqFileRead::start_time(&config,);
    // println!("Start time is {}",start_time);
    // let parsed_time=NextLineTest::FastqFileRead::reframe(&config);
    // println!("Parsed time main:{}",parsed_time);



    // let time_btn=NextLineTest::FastqFileRead::in_btn_time(&config, &mut Collect_barcode);
    // println!("Apna time ayega{:?}",time_btn);


    // mod cli;
    // use cli::arg_passing;

    /*
    input command : cargo run from_time to_time file_name
                    cargo run 1 4 diff_barcode.fastq
    Output should look like this
    MRL summary
    Time barcode05 barcode04
    1hr   0         680
    2hr   896       693.5
    3hr   667       606
    4hr   667       606
    
    */
    

}*/
mod cli;
use cli::Arguments;
use clap::{Parser, Arg};

fn pass_P<P>()-> P
where P:AsRef<PathBuf>+std::clone::Clone+std::marker::Sync+std::marker::Copy+std::marker::Send+ std::convert::AsRef<std::path::Path>,
{
    let new: Arguments<P>=Arguments::parse();
    new.file_path
}
fn main()-> Result<(), Box<dyn std::error::Error>>{

    // let args: Arguments<dyn P::AsRef<PathBuf>+Clone+Send+Sync>::Arguments=Arguments::parse();
    // let args: Arguments<P>=Arguments::parse();
    
    // let file_type:&(dyn AsRef<PathBuf>+Sync+Send+Clone+Copy+AsRef<Path>)=pass_P();
    type P=dyn AsRef<PathBuf>+std::marker::Send+std::marker::Sync+std::marker::Copy;
    type r = dyn AsRef<PathBuf> + Send + Sync + Copy;
    let args:Arguments<P>=Arguments::parse();
    
    // let args:Arguments< dyn AsRef<PathBuf>+Clone+Send+Sync>=Arguments::parse();
    // let from=args.from;
    // let to=args.to;
    // let file=File::open::<&_>(&args.file_path).expect("File not found");
    // let file: Result<File, _> = File::open::(<&args.file_path>);
    // let file= match args.file_path {
    //     Ok(P) =>{println!("Do something:{}",P)},
    //     _=>println!("Error reading file")
    // };
    
    // println!("{:?}",args);
    Ok(())
}