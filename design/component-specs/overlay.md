# Overlay

状态：已实现（P1）

## 用途与边界
为应用内浮层提供语义遮罩和外部点击拦截，不负责窗口层级、焦点陷阱或系统模态行为。

## 公开 API
`active: bool = true`、`dismiss-on-click: bool = true`、`accessible-name: string`；`dismissed()`。

## 状态与交互
支持 active、disabled dismissal；点击遮罩仅在允许时触发一次 dismissed。内容区域由调用方放在其上层并负责阻止事件穿透。

## 无障碍与本地化
Role 为 region，名称由宿主提供；没有内置文案。RTL 不改变遮罩行为。

## Gallery、测试与限制
随 Modal/Toast 组合编译和 Gallery 冒烟验证。Slint 原生 Rectangle/TouchArea 不提供平台模态语义，平台窗口所有权仍待增强层实现。

遵循全局视觉、交互、无障碍和内容本地化规范。
