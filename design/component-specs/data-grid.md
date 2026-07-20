# DataGrid

状态：已实现（P2）

## 用途与边界
承载大数据表格的宿主窗口化、排序、选择、列宽和编辑请求；普通只读数据使用 Table。

## 公开 API
`rows`、`columns`、`row-offset`、`total-row-count`、`current-row`、`editable` 等；排序、选择、编辑、范围请求回调；`request-range`、`select-row`、`request-edit`。

## 状态与交互
复用 StandardTableView。局部行映射为全局索引，编辑和数据窗口均显式请求宿主。

## 无障碍与本地化
使用 table 角色并声明完整行数；单元格格式化由数据层按 Locale 完成。

## Gallery、测试与限制
Gallery 展示 2048 行宿主窗口；Harness 验证范围截断和编辑边界。当前不内建数据缓存、编辑器、固定列或可变行高；虚拟化是宿主窗口化协议。

遵循四份全局规范。
