# ShortcutHint

状态：已实现（P1）。源码：`ui/primitives/shortcut-hint.slint`。

## 用途与边界
用于菜单、命令或帮助文字旁显示跨平台快捷键；不负责注册、解析或执行快捷键。

## 公开 API
`shortcut`、`accessible-name`、`enabled`。

## 状态与交互
覆盖默认、Disabled、长组合、不同平台符号和缩放；组件只读，Hover、Pressed、Focused、Selected、Loading 和 Error 不适用。

## 无障碍与本地化
使用 text 角色；宿主根据平台提供 `Ctrl`、`⌘` 等最终显示和朗读名称。

## Gallery、测试与限制
Gallery 展示 Windows/macOS 风格和禁用；编译与截图验收。不内建平台探测，避免平台 API 泄漏到核心层。

遵循四份全局规范。
