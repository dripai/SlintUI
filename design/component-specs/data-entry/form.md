# Form

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/form.slint`。公开名称：`Form`。

聚合字段错误并统一提交门禁；单个字段布局使用 FormRow，业务校验仍由数据层负责。

## 公开 API

### 数据类型与枚举

#### `FormError`

| 字段或值 | 类型/语义 |
|---|---|
| `field` | `string` |
| `message` | `string` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `errors` | `in` | `[FormError]` | `类型默认值` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `enabled` | `in` | `bool` | `true` | 调用方 | false 时阻止用户操作；程序设置不触发用户事件 |
| `submitting` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `""` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `error-summary-title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `valid` | `out` | `bool` | `errors.length == 0` | 组件派生 | 取值范围、联动和非法值处理见行为规范 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `submitted` | `无` | 用户或等效公开方法明确提交后触发 |
| `invalid-submit` | `first-error-index: int` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |
| `focus-error-requested` | `index: int` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `submit-form()` | `无` | 当前实现约束：`if (!enabled \|\| submitting) { return; } if (valid) { root.submitted(); } else { root.invalid-submit(0); root.focus-error-requested(0); }` |

## 视觉规范

### 组成结构

当前实现由 `VerticalLayout`、`Rectangle`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-surface`、`Theme.border-width`、`Theme.color-error`、`Theme.direction`、`Theme.font-size-body`、`Theme.font-size-caption`、`Theme.font-weight-medium`、`Theme.radius-medium`、`Theme.space-1`、`Theme.space-2`、`Theme.space-3`、`Theme.space-4`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

无错误时提交一次；错误时显示摘要并请求聚焦首项；disabled/submitting 阻止重复提交，不清空输入。

### 无障碍与本地化

根为 groupbox，错误摘要为 assertive live region。字段名和修复文案由本地化层提供，不在组件内拼接业务句子。

### 验证、宿主职责与限制

Gallery 展示字段错误和摘要；Harness 验证无效提交。Slint 无子组件反射，宿主必须把 error index 映射到具体字段焦点。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
