use std::thread;
use warp::Filter;

mod dns;
#[tokio::main]
async fn main() {
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

    let start = warp::path!("start" / String / String)
        .map(|url, key| start(url, key));

    let stop = warp::path("stop").map(|| stop());

    warp::serve(start.or(stop))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn start(url: String, key: String) -> &'static str {
    println!("{} key {}", url, key);
    return "STARTING";
}

fn stop() -> &'static str {
    return "STOPPING";
}
