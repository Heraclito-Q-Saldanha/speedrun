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
    let conn = client.get_multiplexed_async_connection().await?;

    let (tx, rx) = tokio::sync::mpsc::channel::<Message>(32);

    tokio::spawn(async {
        process(conn, rx).await.unwrap();
    });

    let mut set = JoinSet::new();
    for file in args.files {
        dbg!(&file);
        let tx = tx.clone();
        set.spawn(async {
            read_file(file, tx).await.unwrap();
        });
    }
    while let Some(_) = set.join_next().await {}
    Ok(())
}

async fn process(
    mut conn: redis::aio::MultiplexedConnection,
    mut rx: tokio::sync::mpsc::Receiver<Message>,
) -> error::Result<()> {
    let mut len = 0;
    while let Some(Message::FileRead(file_name, l)) = rx.recv().await {
        len += l;
        log::info!("file {file_name}, len: {l}");
        let _: String = conn.set("outputChannel", len).await?;
    }
    Ok(())
}

async fn read_file(path: String, tx: tokio::sync::mpsc::Sender<Message>) -> error::Result<()> {
    let meta = tokio::fs::metadata(&path).await?;
    let len = meta.len();
    let _ = tx.send(Message::FileRead(path, len)).await;
    Ok(())
}
