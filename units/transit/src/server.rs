use warp::Filter;

#[tokio::main]
async fn main() {
    let verify_route = warp::post()
        .and(warp::path("verify"))
        .and(warp::body::bytes()) // Extract body as raw bytes
        .map(|proof_data: warp::hyper::body::Bytes| {
            let proof_data_str = String::from_utf8_lossy(&proof_data);
            println!("Received proof data: {}", proof_data_str);

            warp::reply::with_status("Proof received", warp::http::StatusCode::OK)
        });

    warp::serve(verify_route).run(([127, 0, 0, 1], 8080)).await;
}
