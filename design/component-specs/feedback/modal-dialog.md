# ModalDialog

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/modal-dialog.slint`。公开名称：`ModalDialog`。

独立顶层阻塞决策窗口；普通信息使用页面内反馈，系统文件选择使用平台增强接口。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `dialog-title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `message` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accept-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `cancel-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `danger` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `busy` | `in` | `bool` | `false` | 调用方 | true 时阻止提交、关闭或重复操作，具体见行为规范 |
| `close-on-escape` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `dialog-title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `accepted` | `无` | 用户或等效公开方法明确接受当前内容后触发 |
| `rejected` | `无` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `accept()` | `无` | 当前实现约束：`if (!busy) { root.accepted(); }` |
| `reject()` | `无` | 当前实现约束：`if (!busy) { root.rejected(); root.hide(); }` |

## 视觉规范

### 组成结构

当前实现由 `Rectangle`、`VerticalLayout`、`Text`、`HorizontalLayout`、`Button`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`danger`、`busy`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-elevated`、`Theme.bg-layout`、`Theme.border-default`、`Theme.border-width`、`Theme.dialog-max-width`、`Theme.dialog-min-width`、`Theme.font-size-body`、`Theme.font-size-title-small`、`Theme.font-weight-medium`、`Theme.radius-large`、`Theme.space-2`、`Theme.space-4`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

busy 阻止关闭与重复提交；Esc 按策略拒绝，危险操作使用 danger 样式。窗口内 Tab 顺序由 Slint 焦点系统管理。

### 无障碍与本地化

内容为 groupbox，提供名称和描述；按钮文案由宿主传入，危险确认默认焦点策略由宿主窗口创建流程决定。

### 验证、宿主职责与限制

Gallery 记录宿主打开方式，顶层组件由编译与冒烟覆盖。Slint 1.17.1 核心层不公开跨窗口 owner/modal API，宿主负责生命周期、焦点恢复和多窗口主题同步。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
