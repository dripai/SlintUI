# DescriptionList

状态：已实现（P1）。源码：`ui/data-display/description-list.slint`。

## 用途与边界
用于名称/值形式的对象详情；可编辑字段使用 FormRow，列式大数据使用 Table。

## 公开 API
`items: [DescriptionItem{label,value}]`、`label-width`、`bordered`、`accessible-name`。

## 状态与交互
覆盖有/无边框、空列表、长 label/value、RTL 和缩放；组件只读，交互状态不适用。

## 无障碍与本地化
根使用 groupbox 并暴露项数；标签和值按源码相邻呈现。值格式化和本地化由数据层完成。

## Gallery、测试与限制
Gallery 展示三行带边框详情；编译与截图验收。当前使用单列名称/值布局，不自动转为多列响应式网格。

遵循四份全局规范。
