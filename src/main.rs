use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {

    //input expected as stdout of new sucessful openvpn connection stdout
    //example: sudo openvpn --config ./one.ovpn --script-security 2 | ./target/debug/ovpn-linux-auto-dns
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut writer: io::StdoutLock<'_> = stdout.lock();

    //read from stdin and search for string "DNS" and then add the ip to resolv.conf
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut iter = line.split("dns server 10 address ");
        iter.next();

        for item in iter {
            let mut iter2: std::str::Split<'_, &str> = item.split(",");
            let ip = iter2.next().unwrap().trim();
            writeln!(writer, "Found DNS entry: {}", ip).unwrap();

            //add ip to resolv.conf
            match add_dns_to_resolv_conf(ip) {
                Ok(_) => println!("Successfully added {} to /etc/resolv.conf", ip),
                Err(e) => eprintln!("Error adding {} to /etc/resolv.conf: {}", ip, e),
            }
        
            //message completion
            writeln!(writer, "Complete, Added : {} to resolv.conf. exiting...", ip).unwrap();

            //exit after first ip is found
            break;
        }
    }

    //add ip to resolv.conf
    fn add_dns_to_resolv_conf(ip: &str) -> io::Result<()> {
        let path = Path::new("/etc/resolv.conf");
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
    
        let mut contents = String::new();
        let mut found_nameserver = false;
        let mut found_ip = false;
        while let Some(line) = lines.next() {
            let line = line?;
            if line.starts_with("nameserver") {
                if !found_nameserver {
                    contents.push_str("nameserver ");
                    contents.push_str(ip);
                    contents.push('\n');
                    found_nameserver = true;
                }
                if line.contains(ip) {
                    found_ip = true;
                } else {
                    contents.push_str(&line);
                    contents.push('\n');
                }
            } else {
                contents.push_str(&line);
                contents.push('\n');
            }
        }
    
        if !found_nameserver {
            contents.push_str("nameserver ");
            contents.push_str(ip);
            contents.push('\n');
        }
    
        if !found_ip {
            let mut options = OpenOptions::new();
            options.write(true).truncate(true);
            let mut file = options.open(&path)?;
            file.write_all(contents.as_bytes())?;
        }
    
        Ok(())
    }

}