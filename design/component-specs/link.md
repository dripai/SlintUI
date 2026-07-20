# Link

状态：已实现（P1）。源码：`ui/controls/link.slint`。

## 用途与边界
用于请求宿主导航到目标；普通命令使用 Button。组件不直接打开浏览器或访问平台 API。

## 公开 API
`text`、`target`、`external`、`enabled`、`accessible-name`、`has-focus`；`activate()`、`activated(target)`。

## 状态与交互
覆盖 Default、Hover、Pressed、Focused、Disabled 和外部链接标识；空目标和禁用状态不回调，Enter/Space 与点击一致。

## 无障碍与本地化
Slint 1.17.1 无公开 link 角色，当前使用 button 角色并把 target 作为说明；宿主应提供清晰的本地化链接文字。

## Gallery、测试与限制
Gallery 展示内部、外部和禁用链接；Harness 覆盖重复激活与禁用。导航、权限和安全策略完全由宿主处理。

遵循四份全局规范。
