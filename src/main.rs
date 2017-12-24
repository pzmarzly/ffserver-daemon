use std::net::{TcpListener, Shutdown};
use std::time::Duration;
use std::process::{self, Command, Stdio};
use std::thread;

fn open_connections() -> usize {
    Command::new("/bin/sh").arg("-c").arg("/bin/netstat -ntu | grep :83")
                    .stdin(Stdio::null())
                    .output()
                    .unwrap()
                    .stdout.into_iter()
                    .filter(|c| *c == ('\n' as u8))
                    .count()
}

struct Child {
    server: Option<process::Child>,
    feed: Option<process::Child>
}

impl Child {
    fn new() -> Child {
        Child { server: None, feed: None }
    }
    fn wakeup(&mut self) {
        if let None = self.server {
            self.server = Some(Command::new("/usr/local/bin/ffserver")
                    .arg("-f").arg("/home/pi/ffserver.conf")
                    .arg("-loglevel").arg("quiet")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .spawn()
                    .unwrap());
            thread::sleep(Duration::new(0, 50_000_000));
        }
    }
    fn kill(&mut self) {
        if let Some(ref mut server) = self.server {
            server.kill().unwrap();
            thread::sleep(Duration::new(5, 0));
        }
        if let Some(_) = self.server {
            self.server = None;
        }
    }
    fn keepalive(&mut self) {
        if let Some(_) = self.server {
            let mut should_run = false;
            match self.feed {
                None => should_run = true,
                Some(ref mut x) => {
                    // did it crash? - check return code
                    if let Ok(Some(_)) = x.try_wait() {
                        should_run = true;
                    }
                }
            }
            if should_run {
                self.feed = Some(Command::new("/usr/local/bin/ffmpeg")
                    .arg("-f").arg("v4l2")
                    .arg("-i").arg("/dev/video0")
                    .arg("http://127.0.0.1:90/feed1v.ffm")
                    .arg("-loglevel").arg("quiet")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .spawn()
                    .unwrap())
            }
        }
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        self.kill();
    }
}

fn main() {
    let mut child = Child::new();
    loop {
        let listener = TcpListener::bind("0.0.0.0:83").unwrap();
        for remote in listener.incoming() {
            if let Ok(mut remote) = remote {
                if let Err(_) = remote.shutdown(Shutdown::Both) {
                    //println!("Error disconnecting client");
                }
                break;
            }
        }

        drop(listener);
        child.wakeup();

        let mut n = 0;
        loop {
            n += 1;
            child.keepalive();
            if n % 2000 == 0 {
                if open_connections() == 0 {
                    break;
                }
                if n % 180000 == 0 { // 15 minutes
                    break;
                }
            }
            thread::sleep(Duration::new(0, 50_000_000));
        }

        child.kill();
        thread::sleep(Duration::new(0, 500_000_000));
    }
}
