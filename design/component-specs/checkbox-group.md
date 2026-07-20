# CheckboxGroup

状态：已实现（P1）。源码：`ui/controls/checkbox-group.slint`。

## 用途与边界
用于同一问题下可多选的一组选项；独立布尔值使用 Checkbox，互斥选择使用 RadioGroup。

## 公开 API
`items: [CheckboxGroupItem]`、`enabled`、`accessible-name`、`error-text`；`set-checked()`、`changed(index, checked)`。

## 状态与交互
覆盖 Checked、Unchecked、Focused、Disabled、组错误和长文本；同值、禁用项和越界不回调。

## 无障碍与本地化
根使用 groupbox 并暴露错误说明和项数；条目复用 Checkbox 语义。文案和值分离，显示文字由调用方本地化。

## Gallery、测试与限制
Gallery 展示选中、未选、禁用和错误；Harness 覆盖正常、重复、禁用和越界。首期不提供“全选”业务策略。

遵循四份全局规范。
