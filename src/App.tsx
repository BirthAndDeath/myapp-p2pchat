import { useEffect, useRef } from 'react';

import './App.css';
import ChatInterface from './ChatInterface';
import { Icon } from "@iconify/react";
import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { TrayIcon } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import "./App.css";



import { Routes, Route, Link, useNavigate } from 'react-router-dom';


import { Menu } from '@tauri-apps/api/menu';
async function exitApp() {
  await invoke('exit_app');
}
async function app_init() {


  const menu = await Menu.new({
    items: [
      {
        id: 'settings',
        text: 'Settings',
        action: () => {
          console.log('settings pressed');

        },
      },
    ],
  });

  // 如果某个窗口未显式创建菜单，或者未显式设置菜单，那么此菜单将被分配给它。
  menu.setAsAppMenu().then((res) => {
    console.log('menu set success', res);
  });
}

function App() {
  app_init();

  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }
  // 外部函数实现示例
  const handleSendMessage = (message: string, contactId: number): void => {
    console.log(`发送消息到联系人 ${contactId}: ${message}`);
    // 这里实现实际的消息发送逻辑
    // 例如：调用API发送消息
  };

  const handleGetMessages = (contactId: number) => {
    console.log(`获取联系人 ${contactId} 的消息`);
    // 这里实现实际的消息获取逻辑
    // 例如：从API或本地存储获取消息
    return [
      {
        id: 1,
        text: "这是从外部获取的消息",
        sender: "friend" as const,  // 使用 const 断言确保类型正确
        timestamp: "10:30",
        status: "read" as const  // 使用 const 断言确保类型正确
      },
      {
        id: 2,
        text: "好的，我明白了",
        sender: "me" as const,  // 使用 const 断言确保类型正确
        timestamp: "10:32",
        status: "delivered" as const  // 使用 const 断言确保类型正确
      },
    ];
  };

  const handleTyping = (isTyping: boolean, contactId: number): void => {
    console.log(`联系人 ${contactId} 输入状态: ${isTyping}`);
    // 这里实现实际的输入状态通知逻辑
    // 例如：通过WebSocket通知对方
  };

  return (
    <main className="container">
      <h1><strong>Welcome to Tauri + React - Myapp</strong></h1>
      <p></p>
      <div className="row">
        <a href="https://vite.dev" target="_blank" rel="noopener noreferrer">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank" rel="noopener noreferrer">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank" rel="noopener noreferrer">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
        <a href="./myweb.html" rel="noopener noreferrer">
          <img src={"/myapp.svg"} className="logo myapp" alt="Myapp logo" />
          <Icon icon="mdi-light:home" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <div className="App">
        <h1>简洁聊天界面</h1>
        <div className="app-description">
          <p>这是一个简洁的聊天界面，消息发送和接收逻辑已留空</p>
        </div>

        <div className="chat-wrapper">
          <ChatInterface
            onSendMessage={handleSendMessage}
            onGetMessages={handleGetMessages}
            onTyping={handleTyping}
          />
        </div>

        <div className="implementation-hint">
          <h3>需要实现的外部函数：</h3>
          <ul>
            <li><code>onSendMessage</code>: 处理消息发送逻辑</li>
            <li><code>onGetMessages</code>: 获取联系人消息记录</li>
            <li><code>onTyping</code>: 处理用户输入状态通知</li>
          </ul>
        </div>
      </div>

    </main>
  );
}

export default App;