use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use futures::StreamExt;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use std::io::{IsTerminal, stdout};
use std::net::SocketAddr;
use std::sync::{Arc, OnceLock};
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, mpsc};
pub mod notui;
pub mod tui;

/*
sendmessage /recv todo
*/
// 定义应用状态（Model）
pub struct App {
    // --- 焦点系统 ---
    current_focus: Focus,

    // --- 消息列表组件及其状态 ---
    messages: Vec<String>, // 所有消息
    list_state: ListState,

    // --- 输入框组件 ---
    input: String, // 当前输入的文本

    should_quit: bool,
    local_addr: SocketAddr,
    socket: Arc<UdpSocket>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
// 定义焦点枚举
enum Focus {
    Messages,
    Input,
    SidebarArea,
}

// 3. 实现初始化
impl App {
    pub async fn init() -> Result<Self, std::io::Error> {
        let mut list_state = ListState::default();
        list_state.select(Some(0)); // 默认选中第一条消息

        let socket = Arc::new(UdpSocket::bind("[::]:0").await?); //绑定到所有ipv6
        let local_addr = socket.local_addr()?; //返回实际绑定的地址

        Ok(App {
            current_focus: Focus::Input,
            messages: vec![
                "欢迎使用 chat cli".to_string(),
                "按 Ctrl+Tab 切换焦点，↑↓ 选择消息".to_string(),
                "按 Esc或Ctrl+C 退出应用，在输入框中Ctrl+Enter 发送".to_string(),
            ],
            list_state,
            input: String::new(),
            should_quit: false,
            local_addr,
            socket,
        })
    }
}
