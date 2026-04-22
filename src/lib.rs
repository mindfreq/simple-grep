

pub struct Config {
    search_text: String,
    file_path: String,
    ignor_case: bool,
}

impl Config {
    pub fn build(mut args: Vec<String>) -> () {
        args.remove(0); // Remove program path
        
        println!("{:?}", args);
        // Self {
            
        // }
        ()
    }
    
    fn is_ignore_case(args: Vec<String>) {
        
    }
}