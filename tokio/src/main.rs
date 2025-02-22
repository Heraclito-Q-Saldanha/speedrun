use clap::Parser;
use redis::AsyncCommands;
use tokio::task::JoinSet;

mod error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    #[arg(required = true)]
    files: Vec<String>,
}

#[derive(Debug)]
enum Message {
    FileRead(String, u64),
}

#[tokio::main]
async fn main() -> error::Result<()> {
    simple_logger::init().unwrap();
    let args = Args::parse();
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_multiplexed_async_connection().await?;

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(32);

    let mut set = JoinSet::new();
    for file in args.files {
        let tx = tx.clone();
        set.spawn(async {
            read_file(file, tx).await.unwrap();
        });
    }
    let mut len = 0;
    while let Some(_) = set.join_next().await {
        while let Ok(Message::FileRead(file_name, l)) = rx.try_recv() {
            len += l;
            log::info!("file {file_name}, len: {l}");
            let _: String = conn.set("outputChannel", len).await?;
        }
    }
    Ok(())
}

async fn read_file(path: String, tx: tokio::sync::mpsc::Sender<Message>) -> error::Result<()> {
    let meta = tokio::fs::metadata(&path).await?;
    let len = meta.len();
    let _ = tx.send(Message::FileRead(path, len)).await;
    Ok(())
}
