mod oneshot_handler;
mod serve_handler;

use clap::{arg, Command};
use oneshot_handler::OneshotHandlerFactory;
use serve_handler::ServeHandler;
use ws::connect;

fn main() {
    let cmd = Command::new("platform")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("serve"))
        .subcommand(
            Command::new("mousedown")
                .arg(arg!(<x> "x coordinate").value_parser(clap::value_parser!(u32)))
                .arg(arg!(<y> "y coordinate").value_parser(clap::value_parser!(u32))),
        );

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("serve", _)) => {
            connect("ws://localhost:3012", ServeHandler::new).expect("can't connect");
        }
        Some(("mousedown", sub_matches)) => connect(
            "ws://localhost:3012",
            OneshotHandlerFactory::mouse_down(
                *sub_matches.get_one::<u32>("x").expect("required"),
                *sub_matches.get_one::<u32>("y").expect("required"),
            ),
        )
        .expect("can't connect"),
        _ => {}
    }
}
