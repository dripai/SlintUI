# ButtonGroup

状态：已实现（P1）。源码：`ui/controls/button-group.slint`。

## 用途与边界
用于并列且层级相同的一组操作或模式；单个开关操作使用 ToggleButton，少量互斥视图也可使用 SegmentedControl。

## 公开 API
`items: [ButtonGroupItem]`、`current-index`、`enabled`、`size`、`accessible-name`；`select()`、`select-next()`、`selected()`。

## 状态与交互
覆盖默认、Hover、Pressed、Focused、Selected 和 Disabled；同值、禁用项及越界索引不回调。按钮仍可独立 Tab 聚焦，方向键接入由宿主调用 `select-next()`。

## 无障碍与本地化
根使用 groupbox 角色并暴露项数；条目继承 Button 语义。长文本可省略，调用方提供本地化条目文本。

## Gallery、测试与限制
Gallery“通用 / ButtonGroup”展示选中和整组禁用；Harness 覆盖正常、重复、禁用项和越界。Slint 1.17.1 无法枚举任意 `@children` 做通用 roving focus，因此采用显式模型。

遵循四份全局规范。
