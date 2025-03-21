use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    start_solana_node().await;

    println!("🔗 Теперь можно работать с локальным блокчейном!");

    loop {
        sleep(Duration::from_secs(60)).await;
    }
}

async fn start_solana_node() {
    println!("🚀 Запускаем локальный Solana блокчейн...");

    let output = Command::new("which")
        .arg("solana-test-validator")
        .output()
        .await
        .expect("Не удалось найти solana-test-validator");

    let solana_bin = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let mut child = Command::new(
        solana_bin,
    )
    .arg("--reset")
    .arg("--limit-ledger-size")
    .stdout(Stdio::null()) // Отключаем вывод в консоль
    .stderr(Stdio::null()) // Отключаем вывод ошибок
    .spawn()
    .expect("❌ Не удалось запустить Solana-тестовый узел");

    // Ждем, чтобы нода успела запуститься
    sleep(Duration::from_secs(5)).await;

    println!("✅ Solana-тестовый узел работает на http://127.0.0.1:8899");

    // Ждем завершения процесса (нода будет работать, пока программа не завершится)
    let _ = child.wait().await;
}
