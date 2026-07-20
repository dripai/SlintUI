# Pagination

状态：已实现（P2）

## 用途与边界
控制服务端或宿主分页数据，不负责数据加载。

## 公开 API
`current-page`、`page-count`、`page-size-index`、`page-size-options`、无障碍名称；`changed`、`page-size-changed`；`select-page(page)`。

## 状态与交互
上一页、下一页按边界禁用；程序化页码被限制到有效范围；RTL 图标镜像。

## 无障碍与本地化
根为 groupbox，方向按钮和页码选择均可命名；页大小文本由调用方格式化。

## Gallery、测试与限制
Gallery 和 Harness 覆盖页码、边界和页大小。当前仅显示当前页/总页数，不生成数字页码窗口或跳页输入。

遵循四份全局规范。
