use crate::request::run;

mod articles;
mod error;
mod http;
mod request;
mod route;

fn main() {
    if let Err(e) = async_std::task::block_on(run()) {
        eprintln!("{:?}", e);
    }
}
