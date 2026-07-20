# Label

成熟度：`Alpha`。源码：`crates/slint-ui/ui/primitives/label.slint`。公开名称：`Label`。

用于字段标签和普通说明标签。操作文本使用 Button/Link；补充说明由调用方组合 `Tooltip`，不在 Label 内隐式创建浮层。Slint 1.17.1 没有稳定的跨平台助记键关联 API，因此 P0 不承诺助记键。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `required` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `muted` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

## 视觉规范

### 组成结构

组件本身承担可视结构，不公开额外内部元素；调用方只通过公开属性控制方向和外观。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.direction`、`Theme.font-size-body`、`Theme.font-weight-medium`、`Theme.line-height-body`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

没有独立 Hover、Pressed、Focused、Selected、Disabled 或 Loading 状态；Label 不接收输入。`required` 是供表单组合读取的标记，不自行改写文字；当前 `FormRow` 负责追加可见星号，校验状态由 FormRow 和输入组件表达。

### 无障碍与本地化

角色为 `text`，可访问名称来自可见文本。Placeholder 不替代 Label；Slint 当前无 required 可访问状态，宿主表单必须在说明或错误中表达约束。支持中文、英文、RTL、长文本和文本缩放。

### 验证、宿主职责与限制

Gallery 基础和表单场景覆盖普通、muted、required 组合环境；全局规则见 [`foundations.md`](../../foundations.md)、[`accessibility.md`](../../accessibility.md) 和 [`content-and-localization.md`](../../content-and-localization.md)。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
