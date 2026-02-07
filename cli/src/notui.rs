use crate::App;
use std::net::SocketAddr;
use std::sync::Arc;
pub async fn no_tui_run(app: &mut App) -> std::io::Result<()> {
    let mut targets: Vec<SocketAddr> = vec![];

    //不想写注释了，自己读吧好累（我的命名应该已经很贴切了,大概？）

    println!("输入目标 IPv6 地址（如 [::1]:8000):");
    let mut target_str = String::new();
    std::io::stdin().read_line(&mut target_str).unwrap();
    targets.push(target_str.trim().parse().expect("无效地址"));
    // 接收任务
    todo!();

    // 发送循环
    let mut stdin = String::new();
    println!("输入消息回车发送/Ctrl+C 退出）：");
    loop {
        stdin.clear();
        std::io::stdin().read_line(&mut stdin).unwrap();
        let line = stdin.trim();
        if line.is_empty() {
            continue;
        }
        for target in &targets {
            //&app.socket.send_to(line.as_bytes(), target).await?;
        }
    }
}
