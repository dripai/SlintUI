# CommandPalette

成熟度：`Alpha`。源码：`crates/slint-ui/ui/navigation/command-palette.slint`。公开名称：`CommandPalette`。

展示可搜索命令列表；命令注册、排序、最近使用和快捷键监听属于宿主。

## 公开 API

### 数据类型与枚举

#### `CommandItem`

| 字段或值 | 类型/语义 |
|---|---|
| `text` | `string` |
| `description` | `string` |
| `shortcut` | `string` |
| `enabled` | `bool` |
| `visible` | `bool` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `commands` | `in` | `[CommandItem]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `query` | `in-out` | `string` | `""` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `current-index` | `in-out` | `int` | `-1` | 共享受控状态 | -1 表示无选择；越界值的处理见行为规范 |
| `open` | `in` | `bool` | `false` | 调用方 | 打开状态的所有权和关闭原因见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `placeholder-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `query-changed` | `value: string` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `selected` | `index: int` | 用户选择有效且可用项目后触发一次 |
| `close-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `activate()` | `index: int` | 当前实现约束：`if (index >= 0 && index < commands.length && commands[index].enabled && commands[index].visible) { root.selected(index); }` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`TextField`、`Rectangle`、`HorizontalLayout`、`Text`、`TouchArea`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-elevated`、`Theme.border-default`、`Theme.border-width`、`Theme.command-palette-width`、`Theme.fill-hover`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.radius-large`、`Theme.radius-medium`、`Theme.row-height`、`Theme.selection-bg`、`Theme.space-2`、`Theme.space-3`、`Theme.text-disabled`、`Theme.text-primary`、`Theme.text-secondary`、`Theme.text-tertiary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖可见、选中、Hover、Disabled 和 Escape 关闭请求；筛选结果由 `visible` 控制。

### 无障碍与本地化

根为 groupbox，命令为 list-item，说明与快捷键由调用方本地化和格式化。

### 验证、宿主职责与限制

Gallery 提供打开入口；Harness 验证启用/禁用。当前不内建模糊搜索、全局快捷键、最近命令存储和焦点恢复。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
