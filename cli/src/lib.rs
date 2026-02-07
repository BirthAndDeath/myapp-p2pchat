// 定义应用状态（Model）
#![doc = include_str!("../../README.md")]
use chat_core::ChatCore;
use ratatui::widgets::ListState;
pub mod notui;
pub mod tui;

/*
sendmessage /recv todo
*/

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
    core: ChatCore,
}
#[derive(Debug, Clone, Copy, PartialEq)]
// 定义焦点枚举
enum Focus {
    Messages,
    Input,
    SidebarArea,
}

impl App {
    pub async fn try_init() -> anyhow::Result<App> {
        let mut list_state = ListState::default();
        list_state.select(Some(0)); // 默认选中第一条消息

        let cfg = chat_core::CoreConfig::new("~/.chat_history.db");
        let mut core = chat_core::ChatCore::try_init(&cfg)?;
        core.swarm
            .listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        core.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        core.swarm.listen_on("/ip6/::/udp/0/quic-v1".parse()?)?;
        core.swarm.listen_on("/ip6/::/tcp/0".parse()?)?;

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
            core,
        })
    }
}
