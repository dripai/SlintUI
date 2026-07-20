# Notification

状态：已实现（P2）

## 用途与边界
展示带标题、详情和操作的持久通知；短暂反馈使用 Toast，系统通知属于平台增强。

## 公开 API
标题、消息、`tone: NotificationTone`、操作/关闭文案、dismissible、busy 和名称；`action-requested`、`dismiss-requested`。

## 状态与交互
覆盖 info、success、warning、error、操作 Loading 和可关闭状态；不会自动消失。

## 无障碍与本地化
根为 live region，错误为 assertive，其余为 polite；语义 SVG 与文字共同表达状态。

## Gallery、测试与限制
Gallery“反馈 / Notification”页展示带图标、标题、说明和操作的通知；编译和截图验收。队列、位置、持久化和跨窗口调度由宿主负责，不调用操作系统通知 API。

遵循四份全局规范。
