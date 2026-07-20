# DatePicker

状态：已实现（P2）

## 用途与边界
选择单个公历日期；日期范围、时区和业务禁用规则由宿主处理。

## 公开 API
`value: DateValue`、年/月/日显示选项、对应整数值与索引、状态和无障碍名称；`changed(value)`；`select(year, month, day)`。

## 状态与交互
组合三个原生 Select；拒绝月份 1–12、日期 1–31 之外的程序化值。

## 无障碍与本地化
根为 groupbox，各字段独立命名；显示顺序、选项与格式由调用方按 Locale 提供。

## Gallery、测试与限制
Gallery 和 Harness 覆盖选择与边界。组件不计算月天数、闰年、日期范围、自然语言输入或弹出月历。

遵循四份全局规范。
