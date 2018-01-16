extern crate irc;

use irc::client::prelude::*;

/* Message { tags: None, prefix: Some("edcragg!edcragg@ip"), command: PRIVMSG("#music", "test") } */

fn main() {
    let server = IrcServer::new("config.toml").unwrap();
    server.identify().unwrap();
    server.for_each_incoming(|message| {

//    println!("{:?}", message);

    match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                let tokens: Vec<_> = msg.split(' ').collect();

                for t in tokens {
                    println!("{:?}", t);
                }
                server.send_privmsg(
                    message.response_target().unwrap_or(target), msg
                ).unwrap();
            }
            _ => (),
        }
    }).unwrap()

}
