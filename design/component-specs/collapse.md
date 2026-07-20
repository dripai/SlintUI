# Collapse

状态：已实现（P1）。源码：`ui/data-display/collapse.slint`。

## 用途与边界
用于按标题展开短内容区；主要导航不使用 Collapse，复杂延迟内容由宿主控制模型。

## 公开 API
`items: [CollapseItem{title,content,enabled,expanded}]`、`enabled`、`accessible-name`；`toggle()`、`changed()`。

## 状态与交互
覆盖 Collapsed、Expanded、Hover、Focused semantics、Disabled、长标题/内容和越界；禁用与越界不回调。手风琴式互斥展开由宿主更新模型，组件不暴露无效开关。

## 无障碍与本地化
标题暴露 button、expandable/expanded；内容紧随标题，RTL 使用方向图标。标题与内容由宿主本地化。

## Gallery、测试与限制
Gallery 展示展开、折叠和禁用项；Harness 覆盖重复展开、禁用与越界。Slint 1.17.1 无通用动态 slot 模型，首期内容为字符串模型。

遵循四份全局规范。
