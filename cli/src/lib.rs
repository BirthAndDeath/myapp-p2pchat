use ratatui::widgets::ListState;
use socket2::Socket;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
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
    message_list_state: ListState,

    //contacts:HashMap<id,Socket>;
    contact_list_state: ListState,
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
            message_list_state: list_state,
            contact_list_state: list_state,
            input: String::new(),
            should_quit: false,
            local_addr,
            socket,
        })
    }
}
