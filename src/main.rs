use std::thread;
use std::process::{Command, Child};
use warp::Filter;

mod dns;

static mut DISCRETE: Option<Child> = None;

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

    let start = warp::path!("start" / String)
        .map(|url| unsafe { start(url) });

    let stop = warp::path("stop").map(|| unsafe { stop() });

    warp::serve(start.or(stop))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

unsafe fn start(url: String) -> &'static str {
    println!("{}", url);

    //Twitch URL = rtmp://live.twitch.tv/app/$KEY
    let cmd: &str = &*format!("rtspsrc location=rtsp://localhost:1181/stream latency=100 ! rtph264depay ! queue ! flvmux ! rtmpsink location=\"{} live=1\"", url);
    DISCRETE = Some(Command::new("gst-launch-1.0").arg(&cmd).spawn().expect("FAILED TO START STREAM"));
    return "STARTING";
}

unsafe fn stop() -> &'static str {
    let process = DISCRETE.take();
    if process.is_some() {
        process.unwrap().kill();
    }
    return "STOPPING";
}
