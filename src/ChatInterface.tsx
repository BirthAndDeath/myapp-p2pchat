import React, { useState, useEffect, useRef, KeyboardEvent, ChangeEvent } from 'react';
import './ChatInterface.css';

// ==================== 类型定义 ====================
// 消息状态类型：发送中/已送达/已读
type MessageStatus = 'sending' | 'delivered' | 'read';

// 发送者类型：自己/好友
type SenderType = 'me' | 'friend';

// 联系人状态类型：在线/离开/离线
type ContactStatus = 'online' | 'away' | 'offline';

// 消息接口定义
interface Message {
    id: number;           // 消息唯一标识
    text: string;         // 消息内容
    sender: SenderType;   // 发送者
    timestamp: string;    // 时间戳
    status: MessageStatus; // 消息状态
}

// 联系人接口定义
interface Contact {
    id: number;           // 联系人唯一标识
    name: string;         // 联系人姓名
    avatar: string;       // 头像（使用文字头像）
    status: ContactStatus; // 在线状态
}

// 组件属性接口定义
interface ChatInterfaceProps {
    onSendMessage?: (message: string, contactId: number) => void;  // 发送消息回调
    onGetMessages?: (contactId: number) => Message[];             // 获取消息回调
    onTyping?: (isTyping: boolean, contactId: number) => void;    // 正在输入回调
}

// ==================== 主组件定义 ====================
const ChatInterface: React.FC<ChatInterfaceProps> = ({
    onSendMessage,
    onGetMessages,
    onTyping
}) => {
    // ==================== 状态管理 ====================
    const [messages, setMessages] = useState<Message[]>([]);      // 消息列表
    const [inputValue, setInputValue] = useState<string>("");    // 输入框内容
    const [isTyping, setIsTyping] = useState<boolean>(false);    // 对方是否正在输入

    // 联系人列表（静态数据，实际应用中可能来自API）
    const [contacts] = useState<Contact[]>([
        { id: 1, name: "张三", avatar: "张", status: "online" },
        { id: 2, name: "李四", avatar: "李", status: "online" },
        { id: 3, name: "王五", avatar: "王", status: "away" },
        { id: 4, name: "赵六", avatar: "赵", status: "offline" },
    ]);

    const [activeContact, setActiveContact] = useState<Contact>(contacts[0]);  // 当前选中的联系人

    // ==================== 引用 ====================
    const messagesEndRef = useRef<HTMLDivElement>(null);  // 用于滚动到消息底部的引用

    // ==================== 副作用处理 ====================
    // 监听当前联系人变化，加载对应消息
    useEffect(() => {
        loadMessages();
    }, [activeContact]);

    // 监听消息列表变化，自动滚动到底部
    useEffect(() => {
        scrollToBottom();
    }, [messages]);

    // ==================== 核心功能函数 ====================
    // 滚动到消息列表底部
    const scrollToBottom = (): void => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    };

    // 加载消息：优先使用外部提供的函数，否则使用模拟数据
    const loadMessages = (): void => {
        if (onGetMessages) {
            const loadedMessages = onGetMessages(activeContact.id);
            setMessages(loadedMessages);
        } else {
            // 模拟消息数据（用于演示）
            setMessages([
                {
                    id: 1,
                    text: "你好！",
                    sender: "friend" as SenderType,
                    timestamp: "10:30",
                    status: "read" as MessageStatus
                },
                {
                    id: 2,
                    text: "你好！需要什么帮助？",
                    sender: "me" as SenderType,
                    timestamp: "10:32",
                    status: "read" as MessageStatus
                },
            ]);
        }
    };

    // 发送消息处理函数
    const handleSendMessage = (): void => {
        if (inputValue.trim() === "") return;  // 空消息不发送

        // 创建新消息对象
        const newMessage: Message = {
            id: Date.now(),            // 使用时间戳作为ID
            text: inputValue,
            sender: "me",
            timestamp: getCurrentTime(),
            status: "sending"          // 初始状态为发送中
        };

        // 更新本地消息列表（乐观更新）
        setMessages(prev => [...prev, newMessage]);

        // 调用外部发送函数（如果提供）
        if (onSendMessage) {
            onSendMessage(inputValue, activeContact.id);
        } else {
            // 演示模式：模拟发送过程
            console.log(`发送消息给 ${activeContact.name}: ${inputValue}`);

            // 模拟发送状态更新：500ms后从"发送中"变为"已送达"
            setTimeout(() => {
                setMessages(prev =>
                    prev.map(msg =>
                        msg.id === newMessage.id ? { ...msg, status: "delivered" } : msg
                    )
                );
            }, 500);
        }

        // 清空输入框
        setInputValue("");
    };

    // 处理输入开始（触发"正在输入"状态）
    const handleInputStart = (): void => {
        if (onTyping) {
            // 调用外部回调通知对方正在输入
            onTyping(true, activeContact.id);
        } else {
            // 演示模式：本地模拟对方输入状态
            setIsTyping(true);
            setTimeout(() => setIsTyping(false), 2000);  // 2秒后停止输入状态
        }
    };

    // ==================== 工具函数 ====================
    // 获取当前时间（HH:MM格式）
    const getCurrentTime = (): string => {
        const now = new Date();
        return `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}`;
    };

    // ==================== 事件处理函数 ====================
    // 键盘事件处理：Enter键发送消息，Shift+Enter换行
    const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>): void => {
        if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();  // 阻止默认换行行为
            handleSendMessage();
        }
    };

    // 输入框变化处理
    const handleInputChange = (e: ChangeEvent<HTMLTextAreaElement>): void => {
        setInputValue(e.target.value);
    };

    // 选择联系人处理
    const handleContactSelect = (contact: Contact): void => {
        setActiveContact(contact);  // 切换联系人
        setInputValue("");          // 清空输入框
    };

    // ==================== 状态显示辅助函数 ====================
    // 获取联系人状态文本
    const getStatusText = (status: ContactStatus): string => {
        switch (status) {
            case 'online': return '在线';
            case 'away': return '离开';
            case 'offline': return '离线';
            default: return '';
        }
    };

    // 获取消息状态图标
    const getStatusIcon = (status: MessageStatus): string => {
        switch (status) {
            case 'sending': return '🕐';    // 时钟图标表示发送中
            case 'delivered': return '✓';   // 单勾表示已送达
            case 'read': return '✓✓';       // 双勾表示已读
            default: return '';
        }
    };

    // ==================== 渲染部分 ====================
    return (
        <div className="chat-container">
            {/* 左侧联系人侧边栏 */}
            <div className="sidebar">
                <div className="sidebar-header">
                    <h2>聊天</h2>
                    <div className="user-status">
                        <span className="status-indicator online"></span>
                        <span>在线</span>
                    </div>
                </div>

                {/* 联系人列表 */}
                <div className="contacts-list">
                    {contacts.map(contact => (
                        <div
                            key={contact.id}
                            className={`contact-item ${activeContact.id === contact.id ? 'active' : ''}`}
                            onClick={() => handleContactSelect(contact)}
                        >
                            {/* 联系人头像和状态 */}
                            <div className="contact-avatar">
                                <span>{contact.avatar}</span>
                                <span className={`status-dot ${contact.status}`}></span>
                            </div>
                            {/* 联系人信息 */}
                            <div className="contact-info">
                                <div className="contact-name">{contact.name}</div>
                                <div className="contact-status">
                                    {getStatusText(contact.status)}
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
            </div>

            {/* 右侧主聊天区域 */}
            <div className="main-chat">
                {/* 聊天头部 - 显示当前联系人信息 */}
                <div className="chat-header">
                    <div className="chat-contact-info">
                        <div className="chat-contact-avatar">
                            <span>{activeContact.avatar}</span>
                            <span className={`status-dot ${activeContact.status}`}></span>
                        </div>
                        <div className="chat-contact-details">
                            <div className="chat-contact-name">{activeContact.name}</div>
                            <div className="chat-contact-status">
                                {getStatusText(activeContact.status)}
                            </div>
                        </div>
                    </div>
                </div>

                {/* 消息显示区域 */}
                <div className="messages-container">
                    <div className="messages-list">
                        {/* 消息列表渲染 */}
                        {messages.map(message => (
                            <div
                                key={message.id}
                                className={`message-wrapper ${message.sender === "me" ? "message-sent" : "message-received"}`}
                            >
                                <div className="message">
                                    <div className="message-text">{message.text}</div>
                                    <div className="message-meta">
                                        <span className="message-time">{message.timestamp}</span>
                                        {/* 仅显示自己发送的消息状态 */}
                                        {message.sender === "me" && (
                                            <span className={`message-status ${message.status}`}>
                                                {getStatusIcon(message.status)}
                                            </span>
                                        )}
                                    </div>
                                </div>
                            </div>
                        ))}

                        {/* 对方正在输入指示器 */}
                        {isTyping && (
                            <div className="typing-indicator">
                                <div className="typing-dots">
                                    <div className="dot"></div>
                                    <div className="dot"></div>
                                    <div className="dot"></div>
                                </div>
                                <span className="typing-text">{activeContact.name} 正在输入...</span>
                            </div>
                        )}

                        {/* 用于滚动定位的空div */}
                        <div ref={messagesEndRef} />
                    </div>
                </div>

                {/* 消息输入区域 */}
                <div className="input-area">
                    <div className="message-input-wrapper">
                        <textarea
                            className="message-input"
                            placeholder={`回复 ${activeContact.name}...`}
                            value={inputValue}
                            onChange={handleInputChange}      // 输入变化处理
                            onKeyDown={handleKeyDown}         // 键盘事件处理
                            onFocus={handleInputStart}        // 获取焦点时触发输入状态
                            rows={1}                          // 初始单行
                        />
                    </div>

                    {/* 发送按钮 */}
                    <button
                        className="send-button"
                        onClick={handleSendMessage}
                        disabled={inputValue.trim() === ""}   // 空消息时禁用
                    >
                        发送
                    </button>
                </div>
            </div>
        </div>
    );
};

export default ChatInterface;