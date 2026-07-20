# ToolButton

状态：已实现。源码：`ui/controls/tool-button.slint`。

## 用途与边界

用于 Toolbar 内的普通或 checked 工具操作。独立表单布尔值使用 `Switch`，工具栏外的普通操作使用 `Button`。

## 公开 API

继承 Button 的全部公开属性、`clicked()` 与 `activate()`；默认使用 text variant，可通过 `checkable`/`checked` 表达工具选中态。

## 状态与交互

Default、Hover、Pressed、Focused、Disabled、Loading、Checked 与 Button 一致；点击和 Enter/Space 走同一激活路径，禁用或 Loading 不触发。

## 无障碍与本地化

继承 button、checkable 和 checked 语义。仅图标用法必须提供本地化名称和 Tooltip；同组工具的文字与顺序由 Toolbar 调用方管理。

## Gallery、测试与限制

Gallery“通用 / ToolButton”页展示 checked ToolButton；自动测试验证激活次数及 checked 单次切换。跨子项方向键 roving focus 不在 ToolButton 内实现。遵循四份全局规范。
