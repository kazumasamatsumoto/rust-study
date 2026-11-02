//! ## 課題 03: 小さな実行器（executor）を自作する
//!
//! 課題 02 の `block_on` は、Future を1つしか動かせませんでした。
//! この課題では、**たくさんのタスクを1つのスレッドで並行に動かす**実行器を作ります。
//! tokio の `spawn` と同じことを、ずっと小さく実装します。
//!
//! **課題 02 の `TimerFuture` をテストで使うので、課題 02 を先に解いてください。**
//!
//! ## 全体の仕組み
//!
//! ```text
//!   Spawner ──spawn──→ ┌──────────────────────────┐
//!                      │ 実行待ちキュー (mpsc)     │ ←── Waker が呼ばれると
//!                      │ [Task] [Task] [Task] ...  │      タスクがキューに戻る
//!                      └────────────┬─────────────┘
//!                                   │ recv
//!                                   ▼
//!                      Executor::run: 取り出して poll
//!                        ├ Ready   → タスクは終わり。捨てる
//!                        └ Pending → Future をタスクに戻す。
//!                                    （Waker が呼ばれるまでキューには入らない）
//! ```
//!
//! 鍵になるのは **「タスク自身が Waker になる」** ことです。
//! `Task` に `Wake` を実装し、`wake` されたら自分自身をキューに送り直します。
//! こうすると、「起こされたタスクだけが、もう一度 poll される」ようになります。

use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

/// 型を消したタスクの Future。
///
/// - 色々な型の Future を同じキューに入れるため `dyn Future` にする
/// - `dyn` は大きさが決まらないので `Box` に入れる
/// - poll するには `Pin` が必要なので `Pin<Box<...>>`（`Box::pin` で作れる）
/// - 実行器のスレッドとタイマーのスレッドの間を行き来するので `Send`
type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// 1つのタスク。
pub struct Task {
    /// 実行中の Future。完了したら `None` にする。
    ///
    /// `Mutex` に入れているのは、`Task` が `Arc` で共有され（キューと Waker の両方が持つ）、
    /// かつ別のスレッドから wake されることがあるので、`Sync` である必要があるからです。
    future: Mutex<Option<BoxFuture>>,
    /// 自分をキューに送り直すための送信側
    queue: Sender<Arc<Task>>,
}

impl Wake for Task {
    /// 自分自身（`self`）を、実行待ちキューに送る。
    ///
    /// 送信に失敗する（= 実行器がもう終わっている）場合は、無視して構いません。
    fn wake(self: Arc<Self>) {
        todo!()
    }
}

/// タスクを実行器に渡す側。
#[derive(Clone)]
pub struct Spawner {
    queue: Sender<Arc<Task>>,
}

/// タスクを実行する側。
pub struct Executor {
    queue: Receiver<Arc<Task>>,
}

/// 実行器と、それにタスクを渡す Spawner を作る。
pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (tx, rx) = mpsc::channel();
    (Executor { queue: rx }, Spawner { queue: tx })
}

impl Spawner {
    /// Future をタスクにして、実行待ちキューに入れる。
    ///
    /// `Box::pin(fut)` で `BoxFuture` にし、`Task` を `Arc` で作ってキューに送る。
    pub fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        todo!()
    }

    /// 結果を返す Future をタスクにして、その結果を受け取るための `JoinHandle` を返す。
    ///
    /// `JoinHandle` も Future なので、別のタスクの中で `.await` できます
    /// （`tokio::spawn` が返す `JoinHandle` と同じ考え方です）。
    ///
    /// ヒント:
    /// 1. `JoinState { result: None, waker: None }` を `Arc<Mutex<...>>` で作る
    /// 2. `fut` を包んだ `async move { ... }` を `spawn` する。その中で
    ///    `fut.await` の結果を `result` に入れ、`waker` が保存されていれば wake する
    /// 3. 同じ `Arc` を持つ `JoinHandle` を返す
    pub fn spawn_with_handle<T: Send + 'static>(
        &self,
        fut: impl Future<Output = T> + Send + 'static,
    ) -> JoinHandle<T> {
        todo!()
    }
}

/// `spawn_with_handle` で作ったタスクと、`JoinHandle` が共有する状態。
struct JoinState<T> {
    /// タスクが終わったら、その結果が入る
    result: Option<T>,
    /// 結果を待っている側の Waker
    waker: Option<Waker>,
}

/// タスクの結果を待つための Future。
pub struct JoinHandle<T> {
    state: Arc<Mutex<JoinState<T>>>,
}

impl<T> Future for JoinHandle<T> {
    type Output = T;

    /// `result` が入っていれば取り出して `Ready`、なければ Waker を保存して `Pending`。
    /// （課題 02 の `TimerFuture::poll` と同じ形です）
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        todo!()
    }
}

impl Executor {
    /// キューからタスクを取り出して poll することを、キューが空になり、かつ
    /// **送信側（Spawner と、まだ生きているタスク）が全て消えるまで**くりかえす。
    /// 戻り値は、poll した合計回数（テストで「無駄な poll をしていないか」を確かめるため）。
    ///
    /// 1. `while let Ok(task) = self.queue.recv()` でタスクを取り出す
    ///    （送信側が全て drop されると `recv` が `Err` を返してループが終わる）
    /// 2. `task.future` のロックを取り、`Option` から Future を取り出す（`take`）。
    ///    `None` なら、そのタスクはもう終わっているので飛ばす
    /// 3. `Waker::from(task.clone())` で Waker を作り、Context を作って poll する
    /// 4. `Pending` なら、Future を `Some(...)` に戻しておく（次に wake されたときに続きを poll するため）
    ///
    /// 注意: `run` を呼ぶ前に、`Spawner` を drop しておかないと、ループが終わりません。
    pub fn run(self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ex02_waker_block_on::TimerFuture;
    use crate::run_with_timeout;
    use std::time::{Duration, Instant};

    const HINT: &str = "run の前に Spawner を drop しているか、Task::wake がキューに送っているか、Pending のときに Future をタスクに戻しているかを確認してください。";

    #[test]
    fn test_runs_all_tasks() {
        let (log, polls) = run_with_timeout(5, HINT, || {
            let log = Arc::new(Mutex::new(Vec::new()));
            let (executor, spawner) = new_executor_and_spawner();
            for i in 0..3 {
                let log = log.clone();
                spawner.spawn(async move {
                    log.lock().unwrap().push(i);
                });
            }
            drop(spawner);
            let polls = executor.run();
            let log = log.lock().unwrap().clone();
            (log, polls)
        });
        // 1つのスレッドで、入れた順に実行される
        assert_eq!(log, vec![0, 1, 2]);
        // すぐ終わるタスクは、1回ずつ poll されるだけ
        assert_eq!(polls, 3);
    }

    #[test]
    fn test_tasks_run_concurrently() {
        let (elapsed, polls) = run_with_timeout(5, HINT, || {
            let (executor, spawner) = new_executor_and_spawner();
            let start = Instant::now();
            for _ in 0..3 {
                spawner.spawn(async {
                    TimerFuture::new(Duration::from_millis(100)).await;
                });
            }
            drop(spawner);
            let polls = executor.run();
            (start.elapsed(), polls)
        });
        // 3つのタイマーを同時に待つので、100ms × 3 = 300ms ではなく、100ms 程度で終わる。
        // 順番に実行すると必ず 300ms 以上かかるので、遅いマシンでも誤判定しないよう上限は 290ms にしている。
        assert!(elapsed >= Duration::from_millis(100), "{elapsed:?}");
        assert!(
            elapsed < Duration::from_millis(290),
            "{elapsed:?}: 並行に動いていません"
        );
        // 各タスクは「最初の poll (Pending)」と「起こされた後の poll (Ready)」の2回だけ
        assert_eq!(polls, 6, "起こされていないタスクまで poll していませんか？");
    }

    #[test]
    fn test_join_handle_returns_value() {
        let got = run_with_timeout(5, HINT, || {
            let got = Arc::new(Mutex::new(None));
            let (executor, spawner) = new_executor_and_spawner();

            let handle = spawner.spawn_with_handle(async {
                TimerFuture::new(Duration::from_millis(20)).await;
                21 * 2
            });
            let got2 = got.clone();
            spawner.spawn(async move {
                let v = handle.await;
                *got2.lock().unwrap() = Some(v);
            });
            drop(spawner);
            executor.run();
            *got.lock().unwrap()
        });
        assert_eq!(got, Some(42));
    }

    #[test]
    fn test_spawn_from_inside_task() {
        let log = run_with_timeout(5, HINT, || {
            let log = Arc::new(Mutex::new(Vec::new()));
            let (executor, spawner) = new_executor_and_spawner();
            let inner_spawner = spawner.clone();
            let log2 = log.clone();
            spawner.spawn(async move {
                log2.lock().unwrap().push("outer");
                let log3 = log2.clone();
                // タスクの中から、新しいタスクを spawn できる
                inner_spawner.spawn(async move {
                    log3.lock().unwrap().push("inner");
                });
            });
            drop(spawner);
            executor.run();
            log.lock().unwrap().clone()
        });
        assert_eq!(log, vec!["outer", "inner"]);
    }
}
