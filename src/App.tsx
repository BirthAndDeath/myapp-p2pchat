
import './App.css';

import { Icon } from "@iconify/react";
import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";

import { Routes, Route, Link, useNavigate, Outlet } from 'react-router-dom';
import "./App.css";



import viteUrl from '/vite.svg'
import tauriUrl from '/tauri.svg'
import myappUrl from '/myapp.svg'

import Chatpage from './pages/Chatpage'
const Home = () => {

  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1><strong>Welcome to Tauri + React - Myapp</strong></h1>
      <p></p>
      <div className="row">
        <a href="https://vite.dev" target="_blank" rel="noopener noreferrer">
          <img src={viteUrl} className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank" rel="noopener noreferrer">
          <img src={tauriUrl} className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank" rel="noopener noreferrer">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
        <a href="./myweb.html" rel="noopener noreferrer">
          <img src={myappUrl
          } className="logo myapp" alt="Myapp logo" sizes='120%' />

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
      {/* 应用程序路由配置：定义了主页("/")和关于页面("/about")的路由规则 */}
      {/* 使用嵌套路由，根路径"/"渲染Layout组件，其中包含导航和子路由出口 */}


      <p>Source code: https://github.com/BirthAndDeath/myapp-p2pchat
        Licensed under AGPL-3.0</p>

    </main>)
}



const Layout = () => {
  const navigate = useNavigate();

  const goback = () => navigate(-1);
  const goforward = () => navigate(1);
  return (
    <>
      <nav>
        <button onClick={() => navigate("/")} aria-label="Home"><Icon icon="mdi-light:home" /></button>
        <button onClick={goback} aria-label="Go back">&lt;</button>
        <button onClick={goforward} aria-label="Go forward">&gt;</button>
        <Link to="/">Home</Link> | <Link to="/Chatpage">Chat</Link>
      </nav >
      <hr />
      <Outlet />          {/* 子路由渲染点 */}
    </>
  )
}
const Contacts = <h2>Contacts</h2>

import { Menu } from '@tauri-apps/api/menu';
async function exitApp() {
  //todo : 退出app功能实现，用于后台退出
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

  return (<Routes>
    <Route path="/" element={<Layout />}>
      <Route index element={<Home />} />
      <Route path="Chatpage" element={<Chatpage />} />
    </Route>
  </Routes>


  );
}

export default App;