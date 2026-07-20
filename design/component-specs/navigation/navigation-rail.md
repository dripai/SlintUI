# NavigationRail

成熟度：`Alpha`。源码：`crates/slint-ui/ui/navigation/navigation-rail.slint`。公开名称：`NavigationRail`。

承载少量桌面一级导航，不用于深层树形导航。

## 公开 API

### 数据类型与枚举

#### `NavigationRailItem`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `icon` | `image` |
| `accessible-name` | `string` |
| `enabled` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `items` | `in` | `[NavigationRailItem]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `current-index` | `in-out` | `int` | `0` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `selected` | `index: int` | 用户选择有效且可用项目后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `select()` | `index: int` | 当前实现约束：`if (index >= 0 && index < items.length && items[index].enabled && current-index != index) { root.current-index = index; root.selected(index); }` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Rectangle`、`Image`、`Text`、`TouchArea` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-secondary`、`Theme.border-width`、`Theme.control-height-large`、`Theme.fill-hover`、`Theme.font-size-caption`、`Theme.icon-default`、`Theme.icon-size-medium`、`Theme.line-height-caption`、`Theme.navigation-rail-width`、`Theme.radius-medium`、`Theme.selection-bg`、`Theme.space-1`、`Theme.space-2`、`Theme.text-disabled`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖默认、Hover、Selected、Disabled；受控索引只在有效变化时回调。

### 无障碍与本地化

根为 list，条目暴露图标名称、文本和状态；固定宽度来自 Theme Token。

### 验证、宿主职责与限制

Gallery 展示图标和文字；Harness 验证选择与禁用。当前不含徽标、折叠模式和方向键 roving focus。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
