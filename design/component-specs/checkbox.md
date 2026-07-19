# Checkbox

状态：已实现。源码：`ui/controls/checkbox.slint`。

- 用途：独立或列表中的选择状态。
- API：`text`、`state: CheckState`、`enabled`、`accessible-name`、`toggled(state)`、`toggle()`。
- 行为：unchecked/checked/indeterminate、Hover、Focus、Disabled；Space 切换；用户切换后才触发回调。
- 图标：checked 和 indeterminate 分别使用 `check.svg`、`minus.svg`，不依赖系统字体字形。
- 限制：Slint 1.17.1 公开的 checkbox 可访问状态只有布尔 `checked`，不能表达 mixed；不确定态目前有独立视觉，但平台读屏的 mixed 状态待上游能力。
