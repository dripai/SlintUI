# Statistic

状态：已实现（P2）

## 用途与边界
突出显示单个关键数值及辅助趋势说明，不替代图表。

## 公开 API
`title`、`value`、`prefix`、`suffix`、`detail`、`tone: StatisticTone`、`accessible-name`。

## 状态与交互
非交互组件；支持 neutral、positive、warning、negative 语义色。

## 无障碍与本地化
暴露组合后的 accessible-value；数字、单位、正负号和趋势文案必须由宿主按 Locale 格式化。

## Gallery、测试与限制
Gallery 展示百分比与正向状态；编译和截图验收。当前不内建数字动画、等宽字体选择和趋势图标。

遵循四份全局规范。
