# Radio / RadioGroup

状态：已实现（P1）。源码：`ui/controls/radio.slint`。

## 用途与边界
用于必须至多选择一个值的选项组；多选使用 CheckboxGroup，少量视图模式切换可使用 SegmentedControl。

## 公开 API
Radio：`text`、`value`、`checked`、`enabled`、`select()`、`selected()`。RadioGroup：`items`、`current-index`、`error-text`、`select()`、`select-next()`、`changed()`。

## 状态与交互
覆盖 Unchecked、Checked、Hover、Focused、Disabled、组错误和长文本；同值、禁用项和越界不回调。

## 无障碍与本地化
Radio 使用 radio-button，组使用 groupbox；暴露 checked、enabled、名称和错误说明。显示文字与稳定 value 分离。

## Gallery、测试与限制
Gallery 展示互斥选择和禁用项；Harness 覆盖正常、重复、禁用和越界。Slint 1.17.1 不能统一接管重复子项焦点，方向键由 `select-next()` 协议承接。

遵循四份全局规范。
