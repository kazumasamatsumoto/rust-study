//! 実験: 追記型ログ + ハッシュインデックス (Bitcask 方式の最小実装)
//! cargo run -p fundamentals --example append_log
//!
//! 一時ファイルを OS の一時ディレクトリに作り、最後に削除します。

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// 1レコードのディスク上の形式:
/// [キーの長さ u32][値の長さ u32][キーのバイト列][値のバイト列]
/// 値の長さが u32::MAX のときは「削除」を表す (トゥームストーン)
const TOMBSTONE: u32 = u32::MAX;

struct Store {
    path: PathBuf,
    file: File,
    /// キー → そのキーの最新レコードが始まるファイル内の位置 (バイト)
    index: HashMap<String, u64>,
}

impl Store {
    fn open(path: &Path) -> io::Result<Store> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)?;
        let mut store = Store {
            path: path.to_path_buf(),
            file,
            index: HashMap::new(),
        };
        store.rebuild_index()?;
        Ok(store)
    }

    /// ログを先頭から読み直してインデックスを作る (起動時の復旧)
    fn rebuild_index(&mut self) -> io::Result<()> {
        self.index.clear();
        let mut reader = BufReader::new(File::open(&self.path)?);
        let mut pos = 0u64;
        let mut records = 0;
        loop {
            let mut header = [0u8; 8];
            match reader.read_exact(&mut header) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
            let klen = u32::from_le_bytes(header[0..4].try_into().unwrap());
            let vlen = u32::from_le_bytes(header[4..8].try_into().unwrap());
            let body_len = klen as usize + if vlen == TOMBSTONE { 0 } else { vlen as usize };
            let mut body = vec![0u8; body_len];
            if reader.read_exact(&mut body).is_err() {
                // 書き込みの途中で落ちた、不完全なレコード
                println!("  位置 {pos} に不完全なレコードがあったので、そこで打ち切った");
                break;
            }
            let key = String::from_utf8_lossy(&body[..klen as usize]).into_owned();
            if vlen == TOMBSTONE {
                self.index.remove(&key);
            } else {
                self.index.insert(key, pos);
            }
            pos += 8 + body_len as u64;
            records += 1;
        }
        println!(
            "  インデックスを再構築: レコード {records} 件を読み、生きているキーは {} 件",
            self.index.len()
        );
        // 不完全なレコードが残っていたら切り詰める。
        // そのままにすると、新しいレコードがゴミの後ろに追記され、
        // 次の起動時にゴミの位置で読み込みが止まって、それ以降のデータが失われる。
        let len = fs::metadata(&self.path)?.len();
        if len > pos {
            OpenOptions::new()
                .write(true)
                .open(&self.path)?
                .set_len(pos)?;
            println!("  末尾のゴミ {} バイトを切り詰めた", len - pos);
        }
        Ok(())
    }

    fn append(&mut self, key: &str, value: Option<&str>) -> io::Result<u64> {
        let pos = self.file.seek(SeekFrom::End(0))?;
        let vlen = value.map_or(TOMBSTONE, |v| v.len() as u32);
        let mut rec = Vec::new();
        rec.extend_from_slice(&(key.len() as u32).to_le_bytes());
        rec.extend_from_slice(&vlen.to_le_bytes());
        rec.extend_from_slice(key.as_bytes());
        if let Some(v) = value {
            rec.extend_from_slice(v.as_bytes());
        }
        self.file.write_all(&rec)?;
        Ok(pos)
    }

    fn set(&mut self, key: &str, value: &str) -> io::Result<()> {
        let pos = self.append(key, Some(value))?;
        self.index.insert(key.to_string(), pos);
        Ok(())
    }

    fn remove(&mut self, key: &str) -> io::Result<()> {
        self.append(key, None)?;
        self.index.remove(key);
        Ok(())
    }

    /// インデックスで位置を引き、その位置だけを読む (ファイル全体は読まない)
    fn get(&self, key: &str) -> io::Result<Option<String>> {
        let Some(&pos) = self.index.get(key) else {
            return Ok(None);
        };
        let mut f = File::open(&self.path)?;
        f.seek(SeekFrom::Start(pos))?;
        let mut header = [0u8; 8];
        f.read_exact(&mut header)?;
        let klen = u32::from_le_bytes(header[0..4].try_into().unwrap()) as usize;
        let vlen = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        let mut body = vec![0u8; klen + vlen];
        f.read_exact(&mut body)?;
        Ok(Some(String::from_utf8_lossy(&body[klen..]).into_owned()))
    }

    /// 生きているキーの最新値だけを新しいファイルに書き、差し替える
    fn compact(&mut self) -> io::Result<()> {
        let tmp_path = self.path.with_extension("compact");
        let mut keys: Vec<String> = self.index.keys().cloned().collect();
        keys.sort();
        {
            let mut tmp = Store {
                path: tmp_path.clone(),
                file: OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .read(true)
                    .write(true)
                    .open(&tmp_path)?,
                index: HashMap::new(),
            };
            for k in &keys {
                let v = self.get(k)?.expect("インデックスにあるキーは読めるはず");
                tmp.set(k, &v)?;
            }
            tmp.file.sync_all()?; // 新しいファイルを確実にディスクへ
        }
        // rename は同じファイルシステム内ではアトミックに置き換わる
        fs::rename(&tmp_path, &self.path)?;
        self.file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&self.path)?;
        self.rebuild_index()
    }

    fn file_size(&self) -> io::Result<u64> {
        Ok(fs::metadata(&self.path)?.len())
    }
}

fn hexdump(path: &Path, limit: usize) -> io::Result<()> {
    let bytes = fs::read(path)?;
    for (i, chunk) in bytes.chunks(16).take(limit).enumerate() {
        let hex: Vec<String> = chunk.iter().map(|b| format!("{b:02X}")).collect();
        let ascii: String = chunk
            .iter()
            .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
            .collect();
        println!("  {:04X}: {:<48} {ascii}", i * 16, hex.join(" "));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join(format!("rust_study_log_{}.db", std::process::id()));
    let _ = fs::remove_file(&path);

    println!("== 1. 書き込みは全て末尾への追記 ==");
    let mut store = Store::open(&path)?;
    store.set("lang", "rust")?;
    store.set("level", "1")?;
    println!("set lang=rust, level=1 → ファイルの中身:");
    hexdump(&path, 4)?;

    println!("\n== 2. 上書きしても古いレコードは消えない ==");
    for i in 2..=5 {
        store.set("level", &i.to_string())?;
    }
    store.remove("lang")?;
    println!("level を 2〜5 に上書きし、lang を削除した");
    println!("get(level) = {:?}", store.get("level")?);
    println!("get(lang)  = {:?}", store.get("lang")?);
    println!("インデックス: {:?}", store.index);
    println!(
        "ファイルサイズ: {} バイト (生きている値は level=5 の1件だけなのに)",
        store.file_size()?
    );

    println!("\n== 3. 再起動: ログを読み直してインデックスを復元する ==");
    drop(store);
    let store = Store::open(&path)?;
    println!("get(level) = {:?}", store.get("level")?);

    println!("\n== 4. 書き込み途中でクラッシュした場合 ==");
    {
        // ヘッダだけ書いて本体を書く前に落ちた、という状況を再現する
        let mut f = OpenOptions::new().append(true).open(&path)?;
        f.write_all(&4u32.to_le_bytes())?;
        f.write_all(&100u32.to_le_bytes())?;
        f.write_all(b"ke")?;
    }
    drop(store);
    let mut store = Store::open(&path)?;
    println!(
        "get(level) = {:?}  ← 完了していたデータは無事",
        store.get("level")?
    );

    println!("\n== 5. コンパクション ==");
    for i in 0..1000 {
        store.set("counter", &i.to_string())?;
    }
    let before = store.file_size()?;
    // 再起動しても 1000 件目まで読めることを確認する
    drop(store);
    let mut store = Store::open(&path)?;
    println!("再起動後の get(counter) = {:?}", store.get("counter")?);
    store.compact()?;
    let after = store.file_size()?;
    println!("counter を 1000 回上書きした後: {before} バイト");
    println!("コンパクション後             : {after} バイト");
    println!(
        "get(counter) = {:?}, get(level) = {:?}",
        store.get("counter")?,
        store.get("level")?
    );

    fs::remove_file(&path)?;
    println!("\n一時ファイルを削除しました");
    Ok(())
}
