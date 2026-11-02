//! 実験: システムコールとバッファリング
//! cargo run -p fundamentals --example syscalls --release
//!
//! 一時ファイルを OS の一時ディレクトリに作り、最後に削除します。

use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::time::Instant;

/// 下にある Writer への write 呼び出しの回数を数えるラッパー。
/// File::write は 1回呼ぶごとに write システムコールを 1回発行するので、
/// File を包めば、システムコールの回数を数えられる。
struct CountingWriter<W> {
    inner: W,
    calls: usize,
}

impl<W: Write> Write for CountingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.calls += 1;
        self.inner.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

const LINES: usize = 100_000;
const LINE: &[u8] = b"0123456789\n"; // 11 バイト

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir();
    let path = dir.join(format!("rust_study_syscalls_{}.txt", std::process::id()));
    println!("一時ファイル: {}", path.display());

    #[cfg(unix)]
    {
        use std::os::fd::AsRawFd;
        let f = File::create(&path)?;
        println!("\n== ファイルディスクリプタ ==");
        println!("開いたファイルの fd = {}", f.as_raw_fd());
        println!("(0 = 標準入力, 1 = 標準出力, 2 = 標準エラー出力 は最初から開いている)");
    }

    println!("\n== 11 バイトの write を {LINES} 回 ==");

    // (a) File に直接書く: write のたびにシステムコール
    let mut w = CountingWriter {
        inner: File::create(&path)?,
        calls: 0,
    };
    let start = Instant::now();
    for _ in 0..LINES {
        w.write_all(LINE)?;
    }
    w.flush()?;
    let direct = start.elapsed();
    println!(
        "File に直接       : write システムコール {:>6} 回, {:?}",
        w.calls, direct
    );

    // (b) BufWriter を挟む: 8KiB 貯まるまでメモリに溜め、まとめて書く
    let mut counting = CountingWriter {
        inner: File::create(&path)?,
        calls: 0,
    };
    let start = Instant::now();
    {
        let mut bw = BufWriter::new(&mut counting);
        for _ in 0..LINES {
            bw.write_all(LINE)?;
        }
        bw.flush()?;
    }
    let buffered = start.elapsed();
    println!(
        "BufWriter 経由    : write システムコール {:>6} 回, {:?}",
        counting.calls, buffered
    );
    println!(
        "→ 約 {:.0} 倍速い",
        direct.as_secs_f64() / buffered.as_secs_f64()
    );
    println!("ファイルサイズ: {} バイト", fs::metadata(&path)?.len());

    println!("\n== write が返っても、まだディスクには書かれていない ==");
    let data = vec![b'x'; 8 * 1024 * 1024]; // 8 MiB
    let mut f = File::create(&path)?;
    let start = Instant::now();
    f.write_all(&data)?;
    let t_write = start.elapsed();
    let start = Instant::now();
    f.sync_all()?; // fsync: ページキャッシュの内容をディスクに書き出すまで待つ
    let t_sync = start.elapsed();
    println!("8 MiB の write_all: {t_write:?}  ← メモリ (ページキャッシュ) にコピーしただけ");
    println!("sync_all          : {t_sync:?}  ← 実際にディスクへ書き出すのを待った");

    fs::remove_file(&path)?;
    println!("\n一時ファイルを削除しました");
    Ok(())
}
