use tiny_http::{Header, Response, Server};
extern crate clipboard_master;
extern crate crossclip;
extern crate hyper_sse;
#[macro_use]
extern crate lazy_static;

use std::thread;

lazy_static! {
    static ref SSE: hyper_sse::Server<u8> = hyper_sse::Server::new();
}

use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use crossclip::{Clipboard, SystemClipboard};

fn main() {
    SSE.spawn("[::1]:8001".parse().unwrap());
    let server = Server::http("0.0.0.0:8000").unwrap();

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
        let response = Response::from_string(
            page.to_string().replace(
                "</body>",
                &("\
<script>
        var evtSource = new EventSource('http://[::1]:8001/push/0?".to_owned()
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
