# 命令行启动webview2小程序
使用tauri:wry创建的一个命令行程序，通过参数启动webview2的命令行程序。
## 参数：
* -fullscreen 使用全屏模式,默认关闭，后面不需要加其他参数
* -url http://www.baidu.com 打开的地址，必须传
* -title 窗口标题
* -pop Y/N 是否允许弹出，默认允许
* -theme light/dark 窗口主题，wry有bug现在暂时无效