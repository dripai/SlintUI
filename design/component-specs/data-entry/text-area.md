# TextArea

成熟度：`Alpha`。源码：`crates/slint-ui/ui/controls/text-area.slint`。公开名称：`TextArea`。

评审状态：`2026-07-21 已完成契约评审`。本页 API 表仍记录当前实现；目标差异尚未写入源码。

用于多行自由文本；单行输入使用 TextField，搜索使用 SearchField。

## 公开 API

### 数据类型与枚举

#### `ValidationState`

| 字段或值 | 类型/语义 |
|---|---|
| `normal` | `枚举值` |
| `success` | `枚举值` |
| `warning` | `枚举值` |
| `error` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `text` | `in-out` | `string` | `绑定：editor.text` | 共享受控状态 | 程序赋值不伪造用户编辑事件 |
| `placeholder-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `read-only` | `in` | `bool` | `false` | 调用方 | true 时允许读取和焦点，不接受用户编辑 |
| `max-length` | `in` | `int` | `0` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `show-count` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `validation` | `in` | `ValidationState` | `ValidationState.normal` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `has-focus` | `out` | `bool` | `editor.has-focus` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |
| `character-count` | `out` | `int` | `text.character-count` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `edited` | `text: string` | 用户编辑原始值时触发；程序赋值不触发 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `replace-text()` | `next: string` | 当前实现约束：`if (enabled && !read-only && text != next) { root.text = next; root.edited(next); }` |

### 评审结论（待实现）

- `text` 是唯一可变值，继续使用 `in-out`；`has-focus`、`character-count` 和计划新增的 `over-limit` 为组件派生状态。
- 删除 `replace-text(next)`。宿主赋值直接写 `text`，不应通过方法伪造用户 `edited` 事件。
- 新增 `clear()`、`focus()`、`clear-focus()`、`select-all()`；TextArea 不提供通用 `submit()`，Enter 始终插入换行，业务若需要 Ctrl+Enter 提交应由 Form 或宿主定义。
- `max-length == 0` 表示无上限；大于零时是校验阈值，不静默截断用户或宿主文本。新增 `over-limit: out bool`，定义为 `max-length > 0 && character-count > max-length`。
- `validation` 仍由宿主拥有，`over-limit` 不自动覆盖宿主的 Error/Warning/Success 选择；Form 可以据此决定校验结果。
- `edited(text)` 仅在用户编辑或等效的 `clear()` 真实改变值时触发；宿主赋值、计数更新和校验变化不触发。

## 视觉规范

### 组成结构

当前实现由 `TextEdit`、`Text`、`FocusRing` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`validation`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-control`、`Theme.bg-disabled`、`Theme.border-default`、`Theme.border-width`、`Theme.color-error`、`Theme.color-success`、`Theme.color-warning`、`Theme.control-min-width`、`Theme.focus-outline`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.line-height-caption`、`Theme.radius-medium`、`Theme.space-2`、`Theme.space-4`、`Theme.text-area-min-height`、`Theme.text-tertiary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

覆盖 Default、Focused、Populated、Empty、Disabled、ReadOnly、Success、Warning、Error、滚动和长文本。`max-length` 是显式计数/校验上限，不静默截断输入。

### 已评审事件时序

| 来源 | 前置条件 | 状态变化 | 事件 |
|---|---|---|---|
| 键盘、IME、粘贴、删除等用户编辑 | `enabled && !read-only` 且值真实变化 | 更新 `text`、`character-count`、`over-limit` | `edited(new-text)` 一次 |
| Enter | `enabled && !read-only` | 插入换行 | 作为普通编辑触发 `edited`，不提交 |
| `clear()` | `enabled && !read-only && text != ""` | 清空并更新派生状态 | `edited("")` 一次 |
| 宿主直接设置 `text` | 任意 | 同步值和派生状态 | 无事件 |
| Disabled、ReadOnly 或重复清空 | 条件不满足 | 无 | 无 |

### 已评审方法语义

| 方法 | 前置条件 | 结果与事件 | 幂等与边界 |
|---|---|---|---|
| `clear()` | `enabled && !read-only && text != ""` | 清空并触发 `edited("")` | 空值、禁用、只读时无操作 |
| `focus()` | `enabled` | 聚焦编辑器 | 已聚焦时幂等；ReadOnly 可聚焦 |
| `clear-focus()` | 当前持有焦点 | 清除焦点 | 未聚焦时幂等 |
| `select-all()` | `enabled && text != ""` | 选中全部文本，不改变值 | 空值时无操作；ReadOnly 可选择 |

当前实现差异：仍公开 `replace-text()`，尚无 `clear()`、焦点、全选和 `over-limit`；现有测试把 `replace-text()` 当作用户编辑，必须按新语义重写。Slint 1.17.1 `TextEdit` 原生提供滚动、IME、全选及剪贴板能力，可直接包装公开焦点和全选方法。

### 无障碍与本地化

使用 text-input 角色并暴露值、只读和计数说明；换行、IME、RTL 和文本缩放由 Slint TextEdit 处理。

### 验证、宿主职责与限制

Gallery 展示长文本计数和禁用；Harness 覆盖正常、重复和禁用更新。调整尺寸由外部布局控制，不提供拖拽手柄。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
