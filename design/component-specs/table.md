# Table

状态：已实现（P1）

## 用途与边界
展示可排序、单选的规则二维数据；大数据虚拟化、编辑和复杂列能力属于 DataGrid。

## 公开 API
`rows: [[StandardListViewItem]]`、`columns: [TableColumn]`、`current-row`、`enabled`、`empty-text`；排序、行变化、指针事件回调和 `select-row(index)`。

## 状态与交互
复用 StandardTableView 的表头排序、行选择、键盘焦点和滚动；空模型显示明确空状态，同值和越界选择不产生额外状态。

## 无障碍与本地化
Role 为 table，暴露名称、可用状态和项数。数据格式、数值对齐、空值文案由宿主按 Locale 提供。

## Gallery、测试与限制
Gallery P1 页覆盖双列和交替行。原生控件样式在编译期选择，当前不承诺固定列、虚拟化、自定义单元格或编辑。

遵循四份全局规范。
