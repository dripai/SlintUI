# ConfirmDialog

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/modal-dialog.slint`。公开名称：`ConfirmDialog`。

在不可撤销或高风险操作前确认对象和影响；可撤销操作优先直接执行并提供撤销。

## 公开 API

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `dangerous` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `dialog-title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 ModalDialog |
| `message` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 ModalDialog |
| `accept-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 ModalDialog |
| `cancel-text` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 ModalDialog |
| `danger` | `in` | `bool` | `false` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 ModalDialog |
| `busy` | `in` | `bool` | `false` | 调用方 | true 时阻止提交、关闭或重复操作，具体见行为规范；继承自 ModalDialog |
| `close-on-escape` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范；继承自 ModalDialog |
| `accessible-name` | `in` | `string` | `dialog-title` | 调用方 | 无可见文字的交互组件必须提供本地化名称；继承自 ModalDialog |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `accepted` | `无` | 用户或等效公开方法明确接受当前内容后触发；继承自 ModalDialog |
| `rejected` | `无` | 由该组件定义的有效用户操作或等效公开方法触发；阻止条件见行为规范；继承自 ModalDialog |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `accept()` | `无` | 当前实现约束：`if (!busy) { root.accepted(); }`；继承自 ModalDialog |
| `reject()` | `无` | 当前实现约束：`if (!busy) { root.rejected(); root.hide(); }`；继承自 ModalDialog |

## 视觉规范

### 组成结构

当前实现由 `Rectangle`、`VerticalLayout`、`Text`、`HorizontalLayout`、`Button`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`dangerous`、`danger`、`busy`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-elevated`、`Theme.bg-layout`、`Theme.border-default`、`Theme.border-width`、`Theme.dialog-max-width`、`Theme.dialog-min-width`、`Theme.font-size-body`、`Theme.font-size-title-small`、`Theme.font-weight-medium`、`Theme.radius-large`、`Theme.space-2`、`Theme.space-4`、`Theme.text-primary`、`Theme.text-secondary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

普通与危险确认、busy、Esc 和取消行为继承 ModalDialog；业务必须使用明确的动词和对象。

### 无障碍与本地化

继承 Dialog 名称、描述和焦点要求；危险状态不能只靠红色，标题和正文必须说明后果。

### 验证、宿主职责与限制

与 ModalDialog 共用顶层编译和宿主说明。原生 owner/modal、默认安全按钮和关闭确认流程由宿主集成层完成。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
