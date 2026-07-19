# Select

状态：已实现。源码：`ui/controls/select.slint`。

## 用途与边界

用于从有限字符串模型中单选。可编辑候选使用后续 ComboBox，多选使用后续 MultiSelect；P0 不伪造搜索 Select。

## 公开 API

继承 Slint ComboBox 的 `model: [string]`、`enabled: bool`、`current-index: int`、`current-value: string`、`has-focus`、`selected(value)`；增加 `accessible-name: string = ""`、`size: ControlSize = medium`、`select-index(index)`。

## 状态与交互

Default、Hover、Pressed、Focused、Disabled、展开、当前选择和空模型行为由原生 ComboBox 提供。点击打开 Popup，方向键选择，Enter 打开/确认，Esc 关闭；`select-index()` 仅在启用、索引有效且发生变化时更新并回调。

## 无障碍与本地化

保留 combobox 的 enabled、expanded、value 和展开动作，补充可访问名称。选项由调用方本地化；RTL、字体、Popup 关闭与焦点恢复沿用原生实现。

## Gallery、测试与限制

Gallery“输入”和 Locale 选择展示 Select；自动测试覆盖程序化选择、重复/越界和禁用不变。Slint 1.17.1 不支持搜索，且标准控件样式在编译期选择，不能完全跟随运行时 Theme。遵循四份全局规范。
