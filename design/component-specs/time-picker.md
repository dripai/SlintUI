# TimePicker

状态：已实现（P2）

## 用途与边界
选择本地墙上时间，不包含日期、时区或持续时间语义。

## 公开 API
`value: TimeValue`、时/分/秒选项与索引、`show-seconds`、状态和名称；`changed(value)`；`select(hour, minute, second)`。

## 状态与交互
组合原生 Select；拒绝 24:00 及超出 0–59 的分秒值。

## 无障碍与本地化
根为 groupbox；12/24 小时、步长和本地化标签由宿主提供。

## Gallery、测试与限制
Gallery 和 Harness 覆盖有效与无效值。当前不内建 AM/PM、时区、文本解析和跨午夜范围。

遵循四份全局规范。
