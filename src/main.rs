use clap::{Parser, Subcommand};
use prettytable::{Table, row};
use reqwest::Error;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "yts-cli")]
#[command(about = "List movies from YTS and get magnet links")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List latest movies
    List {
        /// Number of movies to list
        #[arg(short, long, default_value_t = 10)]
        limit: u8,
    },
    /// Search movies by keyword
    Search {
        /// Search query
        query: String,
        /// Number of results
        #[arg(short, long, default_value_t = 10)]
        limit: u8,
    },
}

#[derive(Debug, Deserialize)]
struct ApiResponse { data: Data }
#[derive(Debug, Deserialize)]
struct Data { movies: Option<Vec<Movie>> }
#[derive(Debug, Deserialize)]
struct Movie {
    title: String,
    year: u16,
    torrents: Vec<Torrent>,
}
#[derive(Debug, Deserialize)]
struct Torrent {
    hash: String,
    quality: String,
    #[serde(rename = "type")]
    torrent_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let (url, params) = match &cli.command {
        Commands::List { limit } => ("https://yts.mx/api/v2/list_movies.json", vec![("limit", limit.to_string())]),
        Commands::Search { query, limit } => (
            "https://yts.mx/api/v2/list_movies.json",
            vec![("query_term", query.clone()), ("limit", limit.to_string())],
        ),
    };

    let resp = reqwest::Client::new()
        .get(url)
        .query(&params)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    let movies = resp.data.movies.unwrap_or_default();
    if movies.is_empty() {
        println!("No movies found.");
        return Ok(());
    }

    let mut table = Table::new();
    table.add_row(row!["Title", "Year", "Quality", "Magnet"]);
    for m in movies {
        if let Some(t) = m.torrents.first() {
            let magnet = format!(
                "magnet:?xt=urn:btih:{}&dn={}&tr=udp://tracker.openbittorrent.com:80",
                t.hash, urlencoding::encode(&m.title)
            );
            table.add_row(row![m.title, m.year, format!("{} {}", t.quality, t.torrent_type), magnet]);
        }
    }
    table.printstd();
    Ok(())
}
