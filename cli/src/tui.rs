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
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::interval;

use crate::{App, Focus};
fn tui_render(frame: &mut Frame, app: &App) {
    // 创建布局
    // 水平切分（左右）
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal) // 水平方向：左 | 右
        .constraints([
            Constraint::Percentage(25), // 左侧：联系人/频道列表（25%宽度）
            Constraint::Percentage(75), // 右侧：主区域（75%宽度）
        ])
        .split(frame.area());

    let sidebar_area = horizontal_chunks[0]; // 左侧区域

    // 将右侧区域再垂直切分（上下）
    let right_vertical = Layout::default()
        .direction(Direction::Vertical) // 垂直方向：上 | 下
        .constraints([
            Constraint::Min(5),    // 右上：消息列表（至少5行，占据剩余空间）
            Constraint::Length(3), // 右下：输入框（固定3行）
        ])
        .split(horizontal_chunks[1]);

    let messages_area = right_vertical[0]; // 消息区
    let input_area = right_vertical[1]; // 输入区

    //  渲染消息列表（List 组件）感谢ai帮我写注释（）
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(Text::from(m.clone())))
        .collect();

    let list = List::new(messages)
        .block(
            Block::default()
                .title(" 消息列表 ")
                .borders(Borders::ALL)
                .border_style(match app.current_focus {
                    Focus::Messages => Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                }),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // 渲染带有状态的List
    frame.render_stateful_widget(list, messages_area, &mut app.list_state.clone());

    // 3. 渲染输入框
    let input = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .title(" 输入框 ")
                .borders(Borders::ALL)
                .border_style(match app.current_focus {
                    Focus::Input => Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                }),
        )
        .wrap(Wrap { trim: true }); // 自动换行

    frame.render_widget(input, input_area);

    // 如果焦点在输入框，设置光标位置
    if let Focus::Input = app.current_focus {
        frame.set_cursor_position((input_area.x + app.input.len() as u16 + 1, input_area.y + 1));
    }

    //  渲染状态栏
    let status = match app.current_focus {
        Focus::Messages => format!(
            "ipv6: {} 模式: 浏览消息 (↑/↓选择)",
            app.local_addr.to_string()
        ),
        Focus::Input => format!(
            "ipv6: {} 模式: 输入文本 (Enter发送，Tab切换焦点，Esc退出应用)",
            app.local_addr.to_string()
        ),
        Focus::SidebarArea => format!(
            "ipv6: {} 模式:选择聊天对象↑/↓选择",
            app.local_addr.to_string()
        ),
    };
    let status_bar = Paragraph::new(status).block(Block::default().borders(Borders::TOP));
    frame.render_widget(status_bar, messages_area);
}
fn handle_event(app: &mut App, event: Event) -> std::io::Result<()> {
    match event {
        //状态机
        Event::Key(key) if key.kind == KeyEventKind::Press => match app.current_focus {
            Focus::Messages => handle_messages_focus(app, key.code),
            Focus::Input => handle_input_focus(app, key.code),
            Focus::SidebarArea => handle_sidebar_area_focus(app, key.code),
        },
        _ => {}
    }
    Ok(())
}
fn handle_sidebar_area_focus(app: &mut App, key_code: KeyCode) {}
fn handle_messages_focus(app: &mut App, key_code: KeyCode) {
    let list_len = app.messages.len();
    match key_code {
        KeyCode::Up => {
            if list_len > 0 {
                let i = app.list_state.selected().unwrap_or(0);
                app.list_state.select(Some(i.saturating_sub(1)));
            }
        }
        KeyCode::Down => {
            if list_len > 0 {
                let i = app.list_state.selected().unwrap_or(0);
                app.list_state.select(Some((i + 1).min(list_len - 1)));
            }
        }

        KeyCode::Enter => {
            // 回复选中的消息
            if let Some(i) = app.list_state.selected() {
                if let Some(msg) = app.messages.get(i) {
                    app.input = format!("回复「{}」: ", msg);
                    app.current_focus = Focus::Input;
                }
            }
        }
        _ => {}
    }
}

fn handle_input_focus(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Enter => {
            // 发送消息
            if !app.input.trim().is_empty() {
                app.messages.push(app.input.clone());
                //send
                app.input.clear();
                // 自动滚动到最新消息
                app.list_state.select(Some(app.messages.len() - 1));
            }
        }

        KeyCode::Char(c) => app.input.push(c),
        KeyCode::Backspace => {
            app.input.pop();
        }

        _ => {}
    }
}
impl Focus {
    fn next_focus(self) -> Self {
        match self {
            Focus::Input => Focus::Messages,
            Focus::Messages => Focus::SidebarArea,
            Focus::SidebarArea => Focus::Input, //懒得写复杂轮换了……就这样凑合着吧或者谁来帮我写一下方向轮换
        }
    }
}

///tui模式
pub async fn tui_run(app: &mut App) -> std::io::Result<()> {
    let mut event_stream = crossterm::event::EventStream::new();
    //绑定到ipv6端口
    enable_raw_mode()?;
    let mut targets: Vec<SocketAddr> = vec![];
    // 初始化终端
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let (data_tx, mut data_rx) = mpsc::channel::<(usize, SocketAddr, Vec<u8>)>(100);
    let recv_socket = app.socket.clone();
    tokio::spawn(async move {
        let mut buf = [0u8; 1024]; // 接收缓冲区
        loop {
            match recv_socket.recv_from(&mut buf).await {
                Ok((size, src_addr)) => {
                    // 将接收到的数据和来源地址通过通道发送
                    let data = buf[..size].to_vec();
                    if let Err(_) = data_tx.send((size, src_addr, data)).await {
                        // 通道已关闭，接收任务退出
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("接收数据时出错: {}", e);
                    break;
                }
            }
        }
    });

    loop {
        let mut tick = interval(Duration::from_millis(50));
        tokio::select! {
            biased;
            Some(Ok(event)) = event_stream.next() => {
                if let Event::Key(key)=event
                &&key.kind == KeyEventKind::Press{
                match  key.code {
                    KeyCode::Esc=> break,
                    KeyCode::Tab =>app.current_focus= app.current_focus.next_focus(),
                    _=>{

                    },
                }}
                handle_event(app, event)?;
            }


            Some((size, src_addr, data)) = data_rx.recv() => {
                // 在这里处理或显示接收到的数据
                let text = String::from_utf8_lossy(&data);
                 app.messages.push(format!("\n[网络] 来自 {} 的消息 ({} 字节): {}",
                    src_addr, size, text));

            }
            _ = tick.tick() => {}
            else =>{
                break;
            }

        }
        if app.should_quit {
            break;
        }
        terminal.draw(|frame| tui_render(frame, &app))?;
    }
    disable_raw_mode()?;
    terminal.clear()?;
    terminal.show_cursor()?;
    // 恢复终端
    Ok(())
}
