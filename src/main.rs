use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Instant;

use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Deserialize, ToSchema)]
struct Info {
    id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct APIResponseStruct {
    pub video_id: String,
    pub title: String,
}

lazy_static::lazy_static! {
    static ref API_KEY: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
}

#[utoipa::path(
    get,
    path = "/get/{id}",
    params(
        ("id" = String, Path, description = "YouTube channel or playlist ID"),
        ("type" = Option<String>, Query, description = "Type filter (shorts/short, live/lives/stream/streams, videos/video/long/longs, all)"),
        ("maxresults" = Option<u32>, Query, description = "Maximum results (1-50)")
    ),
    responses(
        (status = 200, description = "List of videos", body = [APIResponseStruct]),
        (status = 404, description = "No videos found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_videos(
    info: web::Path<Info>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let start = Instant::now();
    let id = match query.get("type").map(String::as_str) {
        Some("shorts") | Some("short") => info.id.replacen("UC", "UUSH", 1),
        Some("live") | Some("lives") | Some("stream") | Some("streams") => {
            info.id.replacen("UC", "UULV", 1)
        }
        Some("videos") | Some("video") | Some("long") | Some("longs") => {
            info.id.replacen("UC", "UULF", 1)
        }
        Some("all") => info.id.replacen("UC", "UU", 1),
        _ => info.id.replacen("UC", "UU", 1),
    };

    let max_results = query
        .get("maxresults")
        .and_then(|v| v.parse::<u32>().ok())
        .map(|v| v.clamp(1, 50))
        .unwrap_or(5);

    let url = format!(
        "https://youtube.googleapis.com/youtube/v3/playlistItems?part=snippet&playlistId={}&key={}&maxResults={}",
        id, *API_KEY.lock().unwrap(), max_results
    );

    println!(
        "Fetching videos for playlist {} | Max results: {} | URL: {}",
        id, max_results, url
    );

    let response = match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => {
                        if let Some(items) = data["items"].as_array() {
                            let videos: Vec<_> = items
                                .iter()
                                .filter_map(|item| {
                                    let video_id = item["snippet"]["resourceId"]["videoId"]
                                        .as_str()
                                        .map(|s| s.to_string());
                                    let title =
                                        item["snippet"]["title"].as_str().map(|s| s.to_string());
                                    let published_at = item["snippet"]["publishedAt"]
                                        .as_str()
                                        .map(|s| s.to_string());

                                    match (video_id, title) {
                                        (Some(video_id), Some(title)) => Some(serde_json::json!({
                                            "videoId": video_id,
                                            "title": title,
                                            "publishedAt": published_at.unwrap_or_default(),
                                        })),
                                        _ => None,
                                    }
                                })
                                .collect();

                            let duration = start.elapsed();
                            HttpResponse::Ok()
                                .insert_header((
                                    "X-Response-Time",
                                    format!("{}ms", duration.as_millis()),
                                ))
                                .json(videos)
                        } else {
                            HttpResponse::NotFound()
                                .json(serde_json::json!({ "error": "No videos found" }))
                        }
                    }
                    Err(_) => HttpResponse::InternalServerError()
                        .json(serde_json::json!({ "error": "Failed to parse response" })),
                }
            } else {
                HttpResponse::InternalServerError()
                    .json(serde_json::json!({ "error": response.status().to_string() }))
            }
        }
        Err(_) => HttpResponse::InternalServerError()
            .json(serde_json::json!({ "error": "Request failed" })),
    };

    response
}

// I know there's an easier way to serve static files but this is the quickest and laziest way for me
async fn serve_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("web/index.html"))
}

async fn serve_site_js() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(include_str!("web/site.js"))
}

#[derive(OpenApi)]
#[openapi(paths(get_videos), components(schemas(Info)))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    *API_KEY.lock().unwrap() = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY is not set");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Invalid port number");

    println!("Server is running on http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/", web::get().to(serve_index))
            .route("/index.html", web::get().to(serve_index))
            .route("/site.js", web::get().to(serve_site_js))
            .route("/docs", web::get().to(|| async {
                HttpResponse::Found()
                    .insert_header(("Location", "https://github.com/GalvinPython/latest-uploads-api#latest-youtube-uploads"))
                    .finish()
            }))
            .route("/docs/", web::get().to(|| async {
                HttpResponse::Found()
                    .insert_header(("Location", "https://github.com/GalvinPython/latest-uploads-api#latest-youtube-uploads"))
                    .finish()
            }))
            .route("/get/{id}", web::get().to(get_videos))
            .route("/get/{id}/", web::get().to(get_videos))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
