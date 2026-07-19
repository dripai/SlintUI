# Toast

状态：已实现（P1）

## 用途与边界
展示短暂、非阻塞且可忽略的结果；安全、权限、数据丢失或必须操作的信息使用 Alert/Dialog。

## 公开 API
`messages: [ToastMessage]`、`accessible-name`；`dismiss-requested(id)`。消息含 id、text、detail、tone、duration、dismissible。

## 状态与交互
按模型顺序排队，duration 大于零时自动请求关闭，可显式关闭；模型删除由宿主原子完成，组件不静默修改业务队列。

## 无障碍与本地化
普通消息 polite，错误 assertive；文本与详情由宿主本地化，关闭按钮提供可访问名称。

## Gallery、测试与限制
Gallery 固定展示 success Toast，截图覆盖。Hover/焦点暂停计时受 Slint Timer 公开能力限制，重要内容禁止放入自动关闭 Toast。

遵循四份全局规范。
