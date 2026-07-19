# IconButton

状态：已实现。源码：`ui/controls/icon-button.slint`。

## 用途与边界

用于仅图标的紧凑操作。能稳定显示文字时优先使用 `Button`；工具栏可切换项使用 `ToolButton`。

## 公开 API

继承 Button 的 `icon`、`size`、`enabled`、`loading`、`accessible-name`、`clicked()` 和 `activate()`；增加 `tooltip: string = ""`。默认 text variant、无可见文字、宽高保持方形。

## 状态与交互

Default、Hover、Pressed、Focused、Disabled、Loading 和激活行为与 Button 一致；Selected 仅在调用方启用 checkable 时适用。

## 无障碍与本地化

`accessible-name` 必填且必须本地化，不能依赖 Tooltip 代替；Tooltip 提供可见补充说明。图标资源由调用方按需导入，不隐式加载完整目录。

## Gallery 与测试

Gallery“操作”页展示带 Tooltip 的图标按钮；自动测试验证 `activate()` 单次触发。Tooltip 焦点触发受 Slint 原生能力限制，重要信息不能只放 Tooltip。遵循四份全局规范。
