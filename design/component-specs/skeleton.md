# Skeleton

状态：已实现（P2）

## 用途与边界
首次加载时提供内容轮廓占位；短时局部操作使用 Spinner。

## 公开 API
`rows`、`show-avatar`、`active`、`accessible-name`、`status-text`。

## 状态与交互
非交互组件；活动时在两个 Theme 语义填充色间脉冲，`reduced-motion` 下静止。

## 无障碍与本地化
使用 progress-indicator 角色和 loading 值；必须提供描述加载对象的名称。

## Gallery、测试与限制
Gallery 展示头像加三行骨架；截图强制减少动效。当前没有渐变 shimmer、逐行宽度模型和内容形状自动推断。

遵循四份全局规范。
