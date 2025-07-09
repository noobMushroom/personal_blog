use crate::request::run;

mod articles;
mod error;
mod html;
mod html_elements;
mod http;
mod request;
mod route;
mod session;
mod users;
mod utils;

fn main() {
    if let Err(e) = async_std::task::block_on(run()) {
        eprintln!("{:?}", e);
    }
}
