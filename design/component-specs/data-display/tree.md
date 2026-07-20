# Tree

成熟度：`Alpha`。源码：`crates/slint-ui/ui/data-display/tree.slint`。公开名称：`Tree`。

展示分层对象的展开和单选；扁平列表使用 List，复杂大数据树等待明确性能需求。

## 公开 API

### 数据类型与枚举

#### `TreeItem`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `accessible-name` | `string` |
| `level` | `int` |
| `enabled` | `bool` |
| `visible` | `bool` |
| `has-children` | `bool` |
| `expanded` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `items` | `in-out` | `[TreeItem]` | `类型默认值` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `current-index` | `in-out` | `int` | `-1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `has-focus` | `out` | `bool` | `绑定：focus.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index: int` | 用户选择有效且可用项目后触发一次 |
| `expanded-changed` | `index: int, expanded: bool` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `index: int` | 当前实现约束：`if (enabled && index >= 0 && index < items.length && items[index].visible && items[index].enabled && current-index != index) { root.current-index = index; root.selected(index); }` |
| `toggle()` | `index: int` | 当前实现约束：`if (enabled && index >= 0 && index < items.length && items[index].visible && items[index].enabled && items[index].has-children) { root.items[index].expanded = !root.items[index]...` |

## 视觉规范

### 组成结构

当前实现由 `ScrollView`、`VerticalLayout`、`TreeRow`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.card-min-width`、`Theme.direction`、`Theme.empty-state-min-height`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

支持 Hover、Selected、Focused、Disabled、Expanded；上下/Home/End 选择，Enter/Space 和视觉方向键展开收起。

### 无障碍与本地化

根为 tree，条目为 list-item 并暴露 expanded/enabled/name。方向性 Chevron 在 RTL 镜像。

### 验证、宿主职责与限制

Gallery 展示两级树，Harness 验证选择和展开。宿主提供扁平 visible 模型并负责懒加载、父级收起后的可见性和跳过不可用项。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
