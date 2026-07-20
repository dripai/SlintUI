# FilePicker

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/file-picker.slint`。公开名称：`FilePicker`。

提供路径展示和浏览入口；原生文件对话框属于宿主或平台增强，不在核心组件中静默实现。

## 公开 API

### 数据类型与枚举

#### `ControlSize`

| 字段或值 | 类型/语义 |
|---|---|
| `small` | `枚举值` |
| `medium` | `枚举值` |
| `large` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `path` | `in-out` | `string` | `""` | 共享受控状态 | 取值范围、联动和非法值处理见行为规范 |
| `placeholder-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `browse-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `size` | `in` | `ControlSize` | `ControlSize.medium` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `browse-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |
| `changed` | `path: string` | 组件接受一个不同的新值后触发一次 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `request-browse()` | `无` | 当前实现约束：`if (enabled) { root.browse-requested(); }` |
| `choose()` | `next-path: string` | 当前实现约束：`if (enabled && path != next-path) { root.path = next-path; root.changed(next-path); }` |

## 视觉规范

### 组成结构

当前实现由 `HorizontalLayout`、`TextField`、`Button` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`size`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.control-height-large`、`Theme.control-height-medium`、`Theme.control-height-small`、`Theme.control-min-width`、`Theme.space-2`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Empty、Populated、Focused button、Disabled、长路径和程序化选择；同路径不重复 changed，禁用时不请求或更新。

### 无障碍与本地化

根使用 groupbox，内部只读 TextField 与 Button 保持清晰焦点；浏览文案和名称必须本地化。

### 验证、宿主职责与限制

Gallery 展示路径和浏览入口；Harness 覆盖选择、重复、浏览请求和禁用。过滤器、权限、取消与路径规范化由宿主负责。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
