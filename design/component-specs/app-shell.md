# AppShell

状态：已实现（P1）。源码：`ui/layout/app-shell.slint`。

## 用途与边界
用于标题、导航、内容和状态区域的桌面窗口骨架；不负责真实窗口管理或平台命中测试。

## 公开 API
`title-height`、`navigation-width`、`status-height`、`navigation-visible`；输出四个区域的 x/y/width/height，子项通过 `@children` 使用这些几何值定位。

## 状态与交互
覆盖导航显示/隐藏、LTR/RTL、窄窗口和缩放；交互状态由区域内组件负责。

## 无障碍与本地化
根使用 groupbox；区域内容必须自行声明 landmark 语义。RTL 交换导航物理侧，源码焦点顺序不变。

## Gallery、测试与限制
Gallery 展示完整四区骨架；编译与截图验收。Slint 1.17.1 无命名 slot，采用显式几何输出，不提供平台窗口操作。

遵循四份全局规范。
