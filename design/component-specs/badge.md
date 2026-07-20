# Badge

状态：已实现（P1）。源码：`ui/data-display/badge.slint`。

## 用途与边界
用于紧凑地显示状态点或数量；详细状态文字使用 StatusBar 或 Alert。

## 公开 API
`count`、`max-count`、`show-zero`、`dot`、`tone`、`accessible-name`、只读 `display-text`。

## 状态与交互
覆盖零值隐藏/显示、dot、数量、超上限和五种 tone；组件不接收输入，Hover、Pressed、Focused、Disabled 和 Loading 不适用。

## 无障碍与本地化
使用 text 角色并暴露原始 count；dot 必须提供可访问名称。数字格式化需要时由宿主传入相邻说明。

## Gallery、测试与限制
Gallery 展示数量、99+ 和状态点；编译与截图验收。`count` 是整数，不承担业务单位或复数规则。

遵循四份全局规范。
