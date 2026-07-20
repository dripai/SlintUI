# List / ListItem

状态：已实现（P1）。源码：`ui/data-display/list.slint`。

## 用途与边界
用于单列可选择数据；层级数据使用 Tree，表格列数据使用 Table/DataGrid。

## 公开 API
`ListItemData{text, description, enabled}`；List 的 `items`、`current-index`、`enabled`、`empty-text`、`select()/select-next()`、`selected()`；ListItem 的 `item/selected/focused/activated()`。

## 状态与交互
覆盖 Default、Hover、Pressed、Focused、Selected、Disabled item、Empty、长文本和边界索引；同值与禁用项不回调。

## 无障碍与本地化
根使用 list 并暴露项数，条目使用 list-item。文字和说明由宿主本地化，源码顺序即朗读顺序。

## Gallery、测试与限制
Gallery 展示说明、选中和禁用项；Harness 覆盖正常、重复、禁用与越界。复用 Slint ListView 的重复项视口行为，不承诺可变高度海量数据性能。

遵循四份全局规范。
