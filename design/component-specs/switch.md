# Switch

状态：已实现。源码：`ui/controls/switch.slint`。

## 用途与边界

用于立即生效的开/关设置。需要提交后才生效或表达多选时使用 Checkbox；互斥模式使用 SegmentedControl。

## 公开 API

`text: string = ""`、`checked: bool = false`、`enabled: bool = true`、`size: ControlSize = medium`、`accessible-name: string = text`、`toggled(checked)`、`toggle()`。

## 状态与交互

支持 Default、Hover、Pressed、Focused、Disabled、Checked；不适用 Loading/错误。点击或 Space 切换，在状态更新后触发一次回调；禁用不变化。滑块动画使用 Theme 动效并服从 reduced-motion。

## 无障碍与本地化

角色为 switch，暴露 enabled、checkable、checked 和默认动作。文字由调用方本地化，支持长文本、RTL 和文本缩放；不能只靠轨道颜色表达值。

## Gallery、测试与限制

Gallery“布局”和“输入”页覆盖开关环境；自动测试覆盖正常切换和禁用不变。遵循四份全局规范。
