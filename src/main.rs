use crate::request::run;

mod articles;
mod error;
mod html;
mod http;
mod request;
mod route;
mod session;
mod users;

fn main() {
    if let Err(e) = async_std::task::block_on(run()) {
        eprintln!("{:?}", e);
    }
}
