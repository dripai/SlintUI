# TitleBar

状态：已实现（P1）。源码：`ui/layout/title-bar.slint`。

## 用途与边界
提供自绘桌面标题栏外观、拖动请求和窗口操作槽位；真实拖动、缩放、系统菜单和窗口状态属于宿主/平台增强。

## 公开 API
`title`、`icon`、`active`、按钮显示与 maximized 状态、四个可访问名称；`request-*()`、`drag/minimize/maximize/restore/close-requested()`。

## 状态与交互
覆盖 Active/Inactive、Hover close、Maximized/Restored、按钮隐藏、长标题和双击标题；请求不直接修改系统窗口。

## 无障碍与本地化
根使用 groupbox，窗口按钮分别使用 button；名称必须由宿主按平台语言提供。标题省略但完整名称仍可访问。

## Gallery、测试与限制
Gallery“布局 / TitleBar”展示图标与三种窗口按钮；Harness 覆盖最小化、最大化和关闭请求。平台命中测试、拖动和原生阴影不在核心实现。

遵循四份全局规范。
