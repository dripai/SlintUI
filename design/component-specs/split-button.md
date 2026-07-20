# SplitButton

状态：已实现（P2）

## 用途与边界
组合一个高频主操作和同组低频菜单；多个平级操作使用 ButtonGroup。

## 公开 API
`text`、`variant`、`size`、`enabled`、`loading`、`menu-entries`、无障碍名称；`clicked`、`menu-selected`；`activate()`、`activate-menu(index)`。

## 状态与交互
覆盖 Button 的默认、Hover、Pressed、Focused、Disabled、Loading，以及菜单打开和禁用条目。主操作与菜单回调分离。

## 无障碍与本地化
根为 groupbox，两个按钮分别命名；文本由调用方本地化，方向图标使用 SVG。

## Gallery、测试与限制
Gallery P2 页展示组合；Harness 验证主操作、菜单和禁用项。复用 PopupMenu；暂不提供分裂按钮圆角融合和菜单侧自动翻转。

遵循四份全局规范。
