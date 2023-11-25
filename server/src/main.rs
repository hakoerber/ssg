use axum::{
    http::{StatusCode, Uri},
    serve, Router,
};

use signal_hook::{
    consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM},
    iterator::Signals,
};

use std::{env, net::IpAddr, process, thread};

#[path = "../router.rs"]
mod router;

#[macro_export]
macro_rules! add_route {
    ($router:ident, $path:literal, $content_type:literal, $file:literal) => {{
        use axum::http::header;
        use axum::routing::get;

        let route = $router.route(
            $path,
            get(|| async {
                (
                    [(header::CONTENT_TYPE, $content_type)],
                    include_bytes!($file),
                )
            }),
        );
        route
    }};
}

#[tokio::main]
async fn main() {
    let mut signals = Signals::new(&[SIGINT, SIGTERM, SIGHUP, SIGQUIT]).unwrap();

    thread::spawn(move || {
        for signal in signals.forever() {
            println!("received {signal}, quitting");
            process::exit(1);
        }
    });

    let port = env::args().skip(1).next().unwrap().parse::<u16>().unwrap();

    let app = Router::new()
        .merge(router::router())
        .fallback(|uri: Uri| async move { (StatusCode::NOT_FOUND, format!("No route for {uri}")) });

    let listener = tokio::net::TcpListener::bind((IpAddr::from([0, 0, 0, 0]), port))
        .await
        .unwrap();

    println!("listening on port {port}");
    serve(listener, app).await.unwrap();
}
