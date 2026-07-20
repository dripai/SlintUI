# AutoComplete

状态：已实现（P2）

## 用途与边界
在自由输入时给出建议，不等同于必须从集合中选择的 Select。

## 公开 API
`text`、`suggestions: [AutoCompleteOption]`、占位文案、校验、状态和索引；`edited`、`selected`、`accepted`；`show`、`close`、`choose(index)`。

## 状态与交互
输入触发建议弹层；支持选择、提交、禁用项和错误边框。

## 无障碍与本地化
根为 combobox，建议提供名称与说明；匹配、排序和大小写规则由宿主按 Locale 处理。

## Gallery、测试与限制
Gallery 与 Harness 覆盖建议选择和禁用。当前不内建防抖、异步加载、方向键高亮循环和模糊匹配。

遵循四份全局规范。
