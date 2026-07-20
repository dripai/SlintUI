# Popover

状态：已实现（P1）。源码：`ui/data-display/popover.slint`。

## 用途与边界
用于触发器旁的短交互说明和操作；纯说明使用 Tooltip，菜单命令使用 Dropdown，模态任务使用 Dialog。

## 公开 API
`trigger-text`、`title`、`message`、`action-text`、`close-text`、`enabled`、`open`；`show()/close()/toggle()/activate()`、`action-requested()`、`closed()`。

## 状态与交互
覆盖 Closed、Open、Focused、Disabled、Action、Close、点击外部和 Escape；禁用不打开或激活。

## 无障碍与本地化
触发器暴露 expandable/expanded，弹层转移焦点；所有显示文字必须由调用方本地化。

## Gallery、测试与限制
Gallery 展示标题、长说明和双操作；Harness 覆盖重复动作。Slint 1.17.1 无通用命名 slot 和跨窗口焦点恢复，首期采用显式标题/消息/操作 API。

遵循四份全局规范。
