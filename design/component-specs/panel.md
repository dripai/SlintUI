# Panel

状态：已实现（P1）。源码：`ui/layout/panel.slint`。

## 用途与边界
用于带固定标题区和内容区的桌面面板；强调独立信息块使用 Card，页面骨架使用 AppShell。

## 公开 API
`title`、`description`、`bordered`、`accessible-name` 和 `@children`。

## 状态与交互
支持有/无标题、有/无边框、长标题与内容缩放；本身无 Hover、Pressed、Selected、Disabled、Loading 或 Error。

## 无障碍与本地化
使用 groupbox 角色，标题作为默认名称；标题和描述支持中文、英文、RTL 与换行。

## Gallery、测试与限制
Gallery“布局 / Panel”展示标题、说明、内容和操作；编译与截图验收。动作区由调用方在内容中显式编排。

遵循四份全局规范。
