# Tree

状态：已实现（P1）

## 用途与边界
展示分层对象的展开和单选；扁平列表使用 List，复杂大数据树等待明确性能需求。

## 公开 API
`items: [TreeItem]`、`current-index`、`enabled`、`accessible-name`；`selected`、`expanded-changed`；`select(index)`、`toggle(index)`。

## 状态与交互
支持 Hover、Selected、Focused、Disabled、Expanded；上下/Home/End 选择，Enter/Space 和视觉方向键展开收起。

## 无障碍与本地化
根为 tree，条目为 list-item 并暴露 expanded/enabled/name。方向性 Chevron 在 RTL 镜像。

## Gallery、测试与限制
Gallery 展示两级树，Harness 验证选择和展开。宿主提供扁平 visible 模型并负责懒加载、父级收起后的可见性和跳过不可用项。

遵循四份全局规范。
