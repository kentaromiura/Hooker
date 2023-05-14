use tiny_http::{Header, Response, Server};
extern crate clipboard_master;
extern crate crossclip;
extern crate hyper_sse;
#[macro_use]
extern crate lazy_static;
use std::{path::PathBuf};
use std::thread;
use clap::Parser;
use std::fs;

lazy_static! {
    static ref SSE: hyper_sse::Server<u8> = hyper_sse::Server::new();
}


#[derive(Parser,Default,Debug)]
#[clap(author="Cristian Carlesso <@kentaromiura>", version="v1.0.0", about="Hooker page helper for VNs")]
struct Arguments {
   #[clap(short='c', long="hookpage")]
   page: PathBuf,
   #[clap(short='p', long="webport", default_value_t=8000)]
   port1: u32,
   #[clap(short='s', long="sseport", default_value_t=8001)]
   port2: u32
}



use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use crossclip::{Clipboard, SystemClipboard};

fn main() {
    let args = Arguments::parse();
    SSE.spawn(format!("[::1]:{}", args.port2).parse().unwrap());
    let server = Server::http(format!("0.0.0.0:{}",args.port1)).unwrap();

    use std::io;

    struct Handler {
        latest: String,
    }

    impl ClipboardHandler for Handler {
        fn on_clipboard_change(&mut self) -> CallbackResult {
            let clipboard = SystemClipboard::new().unwrap();
            // TODO: maybe refactor this out.
            self.latest = String::from(clipboard.get_string_contents().unwrap());
            // println!("{:?}", self.latest);
            SSE.push(0, "update", &self.latest).ok();

            CallbackResult::Next
        }

        fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
            eprintln!("Error: {}", error);
            CallbackResult::Next
        }
    }

    thread::spawn(|| {
        let _ = Master::new(Handler {
            latest: String::from(""),
        })
        .run();
    });

    for request in server.incoming_requests() {
        let page = include_str!("Texthooker.html");
        //let mut text = page;
        let unwrap;
        let text = if args.page.exists() {
            // todo handle exceptions like file not accessible etc.
            // if maybe_read.is_err() {
            //     let msg = "Error: page file not found or not accessible";
            //     error(msg);
            // }
            let maybe_read = fs::read_to_string(args.page.as_os_str().to_str().unwrap());
            unwrap = maybe_read.unwrap();
            unwrap.as_str()
        } else {
            page
        };
        let response = Response::from_string(
            text.to_string().replace(
                "</body>",
                &(format!("\
<script>
        var evtSource = new EventSource('http://[::1]:{}/push/0?", args.port2).to_owned()
        // Generate Auth token for SSE EventSource
        + SSE.generate_auth_token(Some(0)).unwrap().as_str()
        + "');
        evtSource.addEventListener('update', event => {
            var p = document.createElement('p');
            p.innerHTML = JSON.parse(event.data);
            document.body.appendChild(p);
        });
    </script>
</body>"),
            ),
        )
        .with_header("Content-Type: text/html".parse::<Header>().unwrap());
        let _ = request.respond(response);
    }
}
