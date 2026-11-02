//! 実験: TCP の上で HTTP を手で話す
//! cargo run -p fundamentals --example tcp_echo
//!
//! 全て 127.0.0.1 (自分自身) の中で完結します。外部のネットワークには出ません。

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread;

fn show_bytes(label: &str, bytes: &[u8]) {
    // 改行コードが見えるように、\r と \n をエスケープして表示する
    let text = String::from_utf8_lossy(bytes)
        .replace('\r', "\\r")
        .replace('\n', "\\n\n");
    println!("---- {label} ({} バイト) ----", bytes.len());
    for line in text.lines() {
        println!("  {line}");
    }
}

fn main() -> std::io::Result<()> {
    println!("== 1. ポート 0 で bind すると、OS が空いているポートを選ぶ ==");
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let server_addr = listener.local_addr()?;
    println!("サーバの待ち受けアドレス: {server_addr}");

    // サーバ: 1つの接続を受け付け、リクエストのヘッダまでを読んで、レスポンスを返す
    let server = thread::spawn(move || -> std::io::Result<()> {
        let (stream, peer) = listener.accept()?;
        println!("[server] 接続を受け付けた。相手のアドレス: {peer}");
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut raw = Vec::new();
        // HTTP/1.1 のヘッダは空行 (\r\n\r\n) で終わる
        loop {
            let mut line = Vec::new();
            let n = reader.read_until(b'\n', &mut line)?;
            raw.extend_from_slice(&line);
            if n == 0 || line == b"\r\n" {
                break;
            }
        }
        show_bytes("[server] 受け取った生のリクエスト", &raw);

        let body = "hello from raw TCP\n";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let mut stream = stream;
        stream.write_all(response.as_bytes())?;
        Ok(())
    });

    // クライアント: HTTP リクエストを「ただの文字列」として送る
    let mut client = TcpStream::connect(server_addr)?;
    println!(
        "[client] 自分のアドレス: {}  ← ポートは OS が自動で割り当てた (エフェメラルポート)",
        client.local_addr()?
    );
    let request = "GET /hello HTTP/1.1\r\nHost: localhost\r\nUser-Agent: tcp_echo\r\n\r\n";
    client.write_all(request.as_bytes())?;

    let mut response = Vec::new();
    // サーバが Connection: close で接続を閉じるまで読む
    client.read_to_end(&mut response)?;
    server.join().unwrap()?;
    show_bytes("[client] 受け取った生のレスポンス", &response);

    println!("\n== 2. TCP はバイトの流れ。メッセージの区切りは保存されない ==");
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let server = thread::spawn(move || -> std::io::Result<Vec<String>> {
        let (mut stream, _) = listener.accept()?;
        let mut chunks = Vec::new();
        let mut buf = [0u8; 7]; // わざと小さなバッファで読む
        loop {
            let n = stream.read(&mut buf)?;
            if n == 0 {
                break;
            }
            chunks.push(String::from_utf8_lossy(&buf[..n]).into_owned());
        }
        Ok(chunks)
    });
    let mut client = TcpStream::connect(addr)?;
    client.write_all(b"first|")?;
    client.write_all(b"second|")?;
    client.write_all(b"third|")?;
    drop(client); // 接続を閉じる → サーバ側の read が 0 を返す
    let chunks = server.join().unwrap()?;
    println!("送信側: write を3回 (\"first|\", \"second|\", \"third|\")");
    println!("受信側: read の結果 = {chunks:?}");
    println!("→ read 1回 = write 1回 ではない。区切りはアプリが自分で決める必要がある");

    println!("\n== 3. UDP はデータグラム。区切りが保存される ==");
    let receiver = UdpSocket::bind("127.0.0.1:0")?;
    let sender = UdpSocket::bind("127.0.0.1:0")?;
    sender.send_to(b"first|", receiver.local_addr()?)?;
    sender.send_to(b"second|", receiver.local_addr()?)?;
    sender.send_to(b"third|", receiver.local_addr()?)?;
    let mut buf = [0u8; 1024]; // 大きなバッファで読んでも…
    for _ in 0..3 {
        let (n, from) = receiver.recv_from(&mut buf)?;
        println!(
            "recv_from: {:?} (送信元 {from})",
            String::from_utf8_lossy(&buf[..n])
        );
    }
    println!(
        "→ 1回の send が 1回の recv にそのまま対応する (ただし UDP は届く保証も順序の保証もない)"
    );
    Ok(())
}
