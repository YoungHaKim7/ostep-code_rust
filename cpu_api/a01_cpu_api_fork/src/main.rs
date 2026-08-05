use std::process;

fn main() {
    println!("hello world (pid:{})", unsafe { libc::getpid() });

    let rc = unsafe { libc::fork() };

    if rc < 0 {
        // fork failed; exit
        eprintln!("fork failed");
        process::exit(1);
    } else if rc == 0 {
        // child (new process)
        println!("hello, I am child (pid:{})", unsafe { libc::getpid() });
    } else {
        // parent goes down this path (original process)
        println!("hello, I am parent of {} (pid:{})", rc, unsafe {
            libc::getpid()
        });
    }
}
