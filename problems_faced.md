# Using test-driven developement process(TDD) for helps for checking  all function output passing or failing
* Testing every function or methods implementation is great way of checking whether your funciton is going correct or not .
* Some time it's very difficult if there is not bug related to any functoin , then this TDD comes to play as one can check individually for every funciton pass of fail.
* 


# Importance of using traits in rust
* From my project's point of view, I first introduced trait when I was testing my function as unit test and now passing another funciton inside that finction of test module.
* As command line project , I created a struct for passing two arguments , one is file I/O handling and second arg is for time frame passing, now these commands are linked directly to the every methods implemented inside impl block , for every method implementation it will act as a seperate object inside the trait which is easy for implementing in other places.
* Using trait object it is easily to pass trait object in different places.

  # getting  ${\color{red}error}$ for using multiple implementation for a trait of single type
  * I have created a trait FastqFileRead, inside this trait method signature defines from a single Struct, but it will through error if you use multiple impl block for same type  or same struct
    ```
    pub trait FastqFileRead {
      fn start_time(self)->PrimitiveDateTime;
      fn end_time(self,,start:PrimitiveDateTime)-> PrimitiveDateTime;
    }
    ```
    ```
    pub struct Config{
      pub time_hr:u64,
      pub file_name:File,
    }
    ```
    ```
    impl FastqFileRead for Config {
      fn end_time(self,start:PrimitiveDateTime) ->PrimitiveDateTime{
          start+Duration::new(self.time_hr*3600, 0)
      }
    }
    impl FastqFileRead for Config {
        fn start_time( self)->PrimitiveDateTime {
            let reader=BufReader::new(self.file_name);
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
                                    return  smallest_datetime.unwrap();
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
    }
    ```
   
Here you can see I have been using two implementation blocks for a single Config struct, this gonna give an error showing ***Conflicting implementation for the type Config***
* Issue will be resloved if you put second method inside a single impl block
* 
  
  # getting ${\color{red}error}$ for using self in  multiple places inside a single method
  * I faced this error when I was using another method inside a method where I have already used ***self*** keyword before implementing that mathod. This will show you cant use self for multiple times.
  * To resolve this issue you have to add ***&*** before ***self*** keyword.
    
    ```
    pub trait FastqFileRead {
      fn start_time(self)->PrimitiveDateTime;
      fn end_time(&self,,start:PrimitiveDateTime)-> PrimitiveDateTime;
      fn get_line(&self)->PrimitiveDateTime ;
    }
    pub struct Config{
      pub time_hr:u64,
      pub file_name:File,
    }
    impl FastqFileRead for Config {
      fn end_time(self,start:PrimitiveDateTime) ->PrimitiveDateTime{
          start+Duration::new(self.time_hr*3600, 0)
      }

      fn start_time(self)->PrimitiveDateTime {
            let reader=BufReader::new(self.file_name);
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
                                    return  smallest_datetime.unwrap();
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
    }    
    ```
