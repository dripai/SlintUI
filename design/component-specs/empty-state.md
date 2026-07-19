# EmptyState

状态：已实现。源码：`ui/feedback/empty-state.slint`。

## 用途与边界

用于无数据、无结果、无权限三类空状态。加载中使用 Progress/Spinner，成功或失败结果使用后续 ResultState；组件不推断业务原因。

## 公开 API

`kind: EmptyStateKind = no-data`（no-data/no-results/no-permission）、`icon: image`、`title: string = ""`、`description: string = ""`、`accessible-name: string = title` 和操作 `@children`。

## 状态与交互

EmptyState 本身不接收输入；操作按钮作为子项保持各自焦点和激活行为。三种 kind 只表达语义与图标色，不内置业务文案或静默替代。

## 无障碍与本地化

角色为 region，标题作为名称，说明作为描述，图标为装饰。调用方必须提供本地化标题、说明和操作；支持长文本、RTL 与文本缩放。

## Gallery、测试与限制

Gallery“反馈”页覆盖 no-data、图标、说明和主要操作；主题、Locale 和缩放由全局切换验证。非交互容器通过编译和截图验收。遵循四份全局规范。
