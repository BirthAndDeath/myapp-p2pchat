import './App.css';

import { Icon } from "@iconify/react";
import { lazy, Suspense, useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { Routes, Route, Link, useNavigate, Outlet } from 'react-router-dom';
import "./App.css";




import myappUrl from '/myapp.svg'


const Chatpage = lazy(() => import('./pages/Chatpage'));
const AddContactShow = lazy(() => import('./components/AddContactShow'));
const AddContactScan = lazy(() => import('./components/AddContactScan'));
const About = lazy(() => import('./pages/About'));






const Home = () => {

  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1><strong>Welcome to Mychat</strong></h1>
      <p></p>
      <div className="row">

        <Link to="/about">
          <img src={myappUrl
          } className="logo myapp" alt="Myapp logo" sizes='150%' />
        </Link>

      </div>
      <p>测试版本，仅测试用</p>

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
        <Link to="/">Home</Link> | <Link to="/Chatpage">Chat</Link> | <Link to="/show">add condact</Link>
      </nav >
      <hr />
      <Outlet />          {/* 子路由渲染点 */}
      <Link to="/about">About</Link> | <>Licensed under AGPL-3.0</>
    </>
  )
}


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

      {
        id: 'exit',
        text: 'Exit',
        action: () => {
          console.log('exit_i pressed');
          exitApp()

        },
      },



    ],
  });

  // 如果某个窗口未显式创建菜单，或者未显式设置菜单，那么此菜单也将被分配给它。
  menu.setAsAppMenu().then((res) => {
    console.log('menu set success', res);
  });
}

const PageLoading = () => (
  <div>
    <div>LOADING . . . </div>
    {/* 可以放一个简单的 spinner */}
  </div>
);
function App() {
  useEffect(() => {
    // 在组件挂载后执行初始化
    const initialize = async () => {
      try {
        await app_init();


      } catch (error) {
        console.error('初始化失败:', error);
      }
    };
    initialize();
  }, []); // 空依赖数组确保只在组件挂载时运行一次

  return (
    <Suspense fallback={<PageLoading />}>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="Chatpage" element={<Chatpage />} />
          <Route path="show" element={<AddContactShow />} />
          <Route path="scan" element={<AddContactScan />} />
          <Route path="about" element={<About />} /> {/* 添加About页面路由 */}
        </Route>
      </Routes>
    </Suspense >


  );
}

export default App;