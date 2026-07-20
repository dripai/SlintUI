# Breadcrumb

状态：已实现（P2）

## 用途与边界
展示层级路径并允许返回祖先层级，不替代 Tabs 或历史导航。

## 公开 API
`items: [BreadcrumbItem]`、`accessible-name`；`selected(index)`；`select(index)`。

## 状态与交互
覆盖可用、Hover、Disabled 和当前末级；末级不可激活。RTL 下分隔箭头镜像。

## 无障碍与本地化
路径为 list，条目提供独立名称和可用状态；调用方提供本地化文本。

## Gallery、测试与限制
Gallery 展示三级路径；Harness 验证末级不触发。当前不内建折叠或溢出菜单，窄区域由宿主放入滚动容器或缩短标签。

遵循四份全局规范。
