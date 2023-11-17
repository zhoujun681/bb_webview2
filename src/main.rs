use std::{env, path::PathBuf};
use wry::{application::{
    dpi::LogicalSize,
    window::{self, Fullscreen, Icon, Theme, Window},
}, webview::WebViewBuilderExtWindows};


fn main() {
    let mut url = "";
    let mut title = "bb";
    let mut canPop = true;
    let mut is_fullscreen = false;
    let mut input_theme = "light";

    let args: Vec<String> = env::args().collect();

    for i in 0..args.len() {
        println!("{}", args[i]);
        if args[i].to_lowercase().starts_with("-url") {
            url = args[i + 1].as_str();
        } else if args[i].to_lowercase().starts_with("-title") {
            title = args[i + 1].as_str();
        } else if args[i].to_lowercase().starts_with("-pop") {
            match args[i + 1].to_lowercase().as_str() {
                "y" => canPop = true,
                "n" => canPop = false,
                _ => canPop = true,
            }
        } else if args[i].to_lowercase().starts_with("-theme") {
            input_theme = args[i + 1].as_str();
        } else if args[i].to_lowercase().starts_with("-fullscreen") {
            is_fullscreen = true;
        }
    }

    let _ = bb(
        url.to_string(),
        title.to_string(),
        canPop,
        input_theme.to_string(),
        is_fullscreen,
    );
}
// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub fn bb(
    url: String,
    title: String,
    canPop: bool,
    input_theme: String,
    is_fullscreen: bool,
) -> wry::Result<()> {
    use wry::{
        application::{
            dpi::LogicalPosition,
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoopBuilder},
            platform::windows::EventLoopBuilderExtWindows,
            window::WindowBuilder
        },
        webview::{WebViewBuilder,WebContext},
    };

    enum UserEvent {
        NewWindow(String),
    }

    let mut theme = Theme::default();

    match input_theme.as_str() {
        "light" => theme = Theme::Light,
        "dark" => theme = Theme::Dark,
        _ => theme = Theme::default(),
    }

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event()
        .with_any_thread(true)
        .with_theme(Some(theme))
        .build();
    let monitor = event_loop.primary_monitor().unwrap();
    let size = monitor.size();
    let scale_factor = monitor.scale_factor();

    // 获取屏幕的宽度和高度
    let screen_width = size.width as f64 / scale_factor;
    let screen_height = size.height as f64 / scale_factor;
    //设置窗口宽度高度
    let window_width = screen_width / 1.5;
    let window_height = screen_height / 1.5;

    // 设置窗口大小为屏幕大小的比例，/2为一半
    let window_size = LogicalSize::new(window_width, window_height);
    // 设置窗口初始位置为屏幕中央
    // let window_position = LogicalPosition::new(screen_width / 4.0, screen_height / 4.0);

    let window_position = LogicalPosition::new(screen_width - window_width, 0.0);

    // 使用嵌入的图像数据
    let img = image::load_from_memory(include_bytes!("../ico/zj.png"))
        .unwrap()
        .into_rgba8();

    let (width, height) = img.dimensions();
    let icon = Icon::from_rgba(img.into_raw(), width, height).expect("Failed to open icon");

    let proxy = event_loop.create_proxy();
    let pid = std::process::id(); //进程id

    let windowbuilder = WindowBuilder::new()
        .with_title(format!("{} - PID:{}", title, pid).as_str())
        .with_theme(Some(theme))

        .with_window_icon(Some(icon));
    let mut window=Window::new(&event_loop).unwrap();
    if is_fullscreen {
         window = windowbuilder
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)?;
    }else {
         window = windowbuilder
        .with_inner_size(window_size)
        .with_position(window_position)
        .build(&event_loop)?;
    }
    let start_html = r#"<!DOCTYPE html>
    <html>
    <head>
        <h1>欢迎使用ZJ的webview2程序，此程序不能直接运行，需要通过参数进行调用。</h1>
        <script type="text/javascript">
            function checkPassword() {
                var password = document.getElementById('password').value;
                if (password == '1') {
                    document.getElementById('message').innerText = "参数：\n-fullscreen 使用全屏模式,默认关闭，后面不需要加其他参数\n-url http://www.baidu.com 打开的地址，必须传\n-title 窗口标题\n-pop Y/N 是否允许弹出，默认允许\n-theme light/dark 窗口主题，wry有bug现在暂时无效";
                } else {
                    document.getElementById('message').innerText = "密码错误";
                }
            }
        </script>
    </head>
    <body>
        <p>请输入密码查看：<input type="password" id="password" />
       <button onclick="checkPassword()">提交</button></p>
        <p id="message"></p>
    </body>
    </html>"#;

    let mut webcontext = WebContext::new(Some(PathBuf::from("C:\\webview2_data"))); //设置webview2数据文件存储位置，不设置会默认创建在程序目录

    let webview = Some(
        WebViewBuilder::new(window)?.with_web_context(&mut webcontext)
            .with_new_window_req_handler(move |uri: String| {
                if !canPop {
                    //弹出窗口时触发，使用本窗口打开弹出的URL，返回false代表拦截
                    let _submitted = proxy.send_event(UserEvent::NewWindow(uri.clone())).is_ok();
                    ClientCertificate
                    false
                } else {
                    true
                }
            }).with_html(start_html).unwrap()
            
            .build()?,
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                //初始新建窗口时触发
                let mut final_url = "".to_string();
                if (!url.starts_with("http")) {
                    final_url = format!("http://{}", url);
                } else {
                    final_url = url.to_owned();
                }
                println!("final_url:{}", final_url);
                webview.as_ref().unwrap().load_url(final_url.as_str());
                
                println!("Wry has started!")
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // webview.as_ref().unwrap().window().set_visible(false);
                webview.as_ref().take();

                *control_flow = ControlFlow::Exit
            }
            Event::UserEvent(UserEvent::NewWindow(uri)) => {
                //收到弹出窗口时进行跳转
                webview.as_ref().unwrap().load_url(uri.as_str());
                println!("New Window: {}", uri);
            }
            _ => (),
        }
    });
}
