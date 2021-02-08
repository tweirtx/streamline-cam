use std::thread;

mod dns;

fn main() {
    let dnsargs = dns::Opt {
        multicast_group: "239.255.70.77".parse().unwrap(),
        host: "0.0.0.0".parse().unwrap(),
        port: 50765,
        command: dns::Command::Broadcast { name: Some("streamline-cam".parse().unwrap()) }
    };
    thread::spawn(move || {
        dns::run(dnsargs)
    });
    println!("Online");
    // now that we're broadcasting, start a web server to receive API calls to start/stop streaming
}
