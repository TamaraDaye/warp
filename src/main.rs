#![allow(unused)]

use std::{io, env};
use std::error::Error;

use cli_tool::*;


fn main() -> Result<(), Box<dyn Error>>{
    // if let Ok(args) = parse_args() {
    //     println!("{:?}", args)
    // }

    test_enum();
    Ok(())

}

// fn parse_args() -> Result<Vec<String>, io::Error> {
//     let mut args_iter = env::args().skip(1);
//     todo!()
// //BUG:
// }

fn start(conifg: Config) -> Result<WarpApp, io::Error> {
    todo!()
}

fn get_files(files: Vec<String>) -> Result<Vec<FileData>, io::Error>{
    todo!()
}


fn test_enum (){
    let mut code = ResultCode::Ok(200);
    if let ResultCode::Ok(val) = code {
        print!("{val}");
    }
}

