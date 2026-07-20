# NavigationRail

状态：已实现（P2）

## 用途与边界
承载少量桌面一级导航，不用于深层树形导航。

## 公开 API
`items: [NavigationRailItem]`、`current-index`、`accessible-name`；`selected(index)`；`select(index)`。

## 状态与交互
覆盖默认、Hover、Selected、Disabled；受控索引只在有效变化时回调。

## 无障碍与本地化
根为 list，条目暴露图标名称、文本和状态；固定宽度来自 Theme Token。

## Gallery、测试与限制
Gallery 展示图标和文字；Harness 验证选择与禁用。当前不含徽标、折叠模式和方向键 roving focus。

遵循四份全局规范。
