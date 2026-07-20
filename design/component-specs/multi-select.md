# MultiSelect

状态：已实现（P2）

## 用途与边界
从有限候选集中选择多个值；复杂实体搜索由 AutoComplete 与业务数据层组合。

## 公开 API
`options: [MultiSelectOption]`、`selected-labels`、`selected-summary`、占位文案、尺寸和状态；`selection-requested`；`show`、`close`、`request-selection`。

## 状态与交互
覆盖默认、打开、Hover、Checked、Disabled；模型由宿主更新，不静默修改数据。

## 无障碍与本地化
根为 combobox，选项暴露 checked；摘要和文案由调用方按 Locale 生成。

## Gallery、测试与限制
Gallery 和 Harness 覆盖多选与禁用。当前以摘要显示已选项，不绘制独立 Tag、不内建搜索和 Tag 溢出计算。

遵循四份全局规范。
