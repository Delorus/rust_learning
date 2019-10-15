mod modules;

fn main() {
    call_pub_modules();
}

fn call_pub_modules() {
    modules::client::connect();
}

use modules::client;

fn use_pub_modules() {
    client::connect();
}