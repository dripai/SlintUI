# FormRow

成熟度：`Alpha`。源码：`crates/slint-ui/ui/layout/form-row.slint`。公开名称：`FormRow`。

用于统一字段 Label、输入控件、帮助和错误信息的垂直关系。跨字段提交、校验聚合和错误焦点定位属于后续 `Form`。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `label` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `help-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `error-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `required` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `label` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

## 视觉规范

### 组成结构

当前实现由 `Label`、`Text` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：由内容和全局 Theme 决定。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.color-error`、`Theme.control-height-medium`、`Theme.font-size-caption`、`Theme.line-height-body`、`Theme.line-height-caption`、`Theme.space-1`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

FormRow 本身不接收输入；错误文本优先于帮助文本，错误不会清空或修正子控件值。高度随密度、文本缩放和反馈内容增长。

### 无障碍与本地化

角色为 `groupbox`，名称来自 label，描述优先使用错误、其次帮助文本。业务必须提供已本地化文案；支持长文本和 RTL。`required` 视觉星号不替代业务校验或读屏说明。

### 验证、宿主职责与限制

Gallery“数据录入 / FormRow”页展示标签、帮助文本和输入控件组合；错误与长文本由组件测试和全局环境验证。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
