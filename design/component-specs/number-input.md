# NumberInput

状态：已实现（P1）。源码：`ui/controls/number-input.slint`。

## 用途与边界
用于有上下界和步长的整数输入；自由格式数字、货币和单位格式化由宿主层处理。

## 公开 API
继承 SpinBox 的 `value`、`minimum`、`maximum`、`step-size`、`enabled`、`read-only`、`edited()`；新增 `size`、`accessible-name`、`set-value()`、`step-by()`。

## 状态与交互
覆盖 Default、Hover、Focused、Disabled、ReadOnly、上下界和步进；程序化设置夹取到范围，同值不重复回调。

## 无障碍与本地化
复用 Slint 原生 spinbox 角色、范围、步长和增减动作。Locale 格式化不在组件内完成。

## Gallery、测试与限制
Gallery 展示范围、步长和禁用；Harness 覆盖重复、越界夹取与禁用。当前只支持整数，这是 Slint 原生 SpinBox 的公开类型边界。

遵循四份全局规范。
