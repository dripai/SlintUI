# Calendar

状态：已实现（P2）

## 用途与边界
展示月视图并选择单日；排期、范围选择和日程数据由更高层组合。

## 公开 API
`weekday-labels`、`days: [CalendarDay]`、`title`、名称；`selected`、`previous-requested`、`next-requested`；`select(index)`。

## 状态与交互
覆盖当前月、非当前月、Today、Selected、Hover、Disabled；日期模型完全受控。

## 无障碍与本地化
根为 groupbox，每日提供完整 accessible-name；周起始日、标题和数字由宿主按 Locale/历法生成。

## Gallery、测试与限制
Gallery 与 Harness 覆盖选择和禁用。组件不计算 42 格日期、农历、范围选择或键盘二维漫游，当前固定七列六行几何。

遵循四份全局规范。
