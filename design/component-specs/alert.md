# Alert

状态：已实现（P1）。源码：`ui/feedback/alert.slint`。

## 用途与边界
用于页面内必须可见的 info、success、warning、error 消息；短暂反馈使用 Toast，持久浮层使用 Notification。

## 公开 API
`title`、`message`、`tone`、`dismissible`、`action-text`、`enabled`、名称属性；`activate()/dismiss()`、`action-requested()`、`dismiss-requested()`。

## 状态与交互
覆盖四种 tone、带/不带标题、操作、可关闭、Disabled 和长文本；禁用阻止操作与关闭。

## 无障碍与本地化
使用 region 和 polite/assertive live region；图标不重复朗读，标题、消息、操作和关闭名称必须本地化。

## Gallery、测试与限制
Gallery 展示 info 与 error；Harness 覆盖操作、关闭和禁用。关闭仅发请求，宿主决定移除和持久化。

遵循四份全局规范。
