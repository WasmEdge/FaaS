#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to this WebAssembly (Wasm) Function as a Service (FaaS) API. Please see the official documentation < https://github.com/WasmEdge/FaaS/tree/main/documentation/usage.md >."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}