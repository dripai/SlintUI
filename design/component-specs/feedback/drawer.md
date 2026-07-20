# Drawer

成熟度：`Alpha`。源码：`crates/slint-ui/ui/feedback/drawer.slint`。公开名称：`Drawer`。

在当前窗口侧边承载辅助任务；必须打断流程时使用 ModalDialog。

## 公开 API

### 数据类型与枚举

#### `DrawerSide`

| 字段或值 | 类型/语义 |
|---|---|
| `start` | `枚举值` |
| `end` | `枚举值` |

### 属性

| 名称 | 方向 | 类型 | 默认值 | 所有者 | 说明与边界 |
|---|---|---|---|---|---|
| `open` | `in` | `bool` | `false` | 调用方 | 打开状态的所有权和关闭原因见行为规范 |
| `side` | `in` | `DrawerSide` | `DrawerSide.end` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `panel-width` | `in` | `length` | `Theme.drawer-width` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `title` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `accessible-name` | `in` | `string` | `title` | 调用方 | 无可见文字的交互组件必须提供本地化名称 |
| `close-accessible-name` | `in` | `string` | `""` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `dismiss-on-mask` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |
| `close-on-escape` | `in` | `bool` | `true` | 调用方 | 取值范围、联动和非法值处理见行为规范 |

### 内容入口

`@children` 插入到组件声明的内容区域；尺寸、裁剪和焦点顺序由该区域布局与行为规范约束。

### 事件

| 名称 | 参数 | 触发语义与边界 |
|---|---|---|
| `close-requested` | `无` | 请求宿主执行操作，不表示操作已经成功 |

### 公开方法

| 名称 | 参数 | 状态、事件与边界 |
|---|---|---|
| `request-close()` | `无` | 当前实现约束：`if (open) { root.close-requested(); }` |

## 视觉规范

### 组成结构

当前实现由 `TouchArea`、`Rectangle`、`VerticalLayout`、`HorizontalLayout`、`Text`、`Image`、`FocusScope` 组成。内部元素不是公开 API，调用方不得依赖其名称或坐标。

### 变体、尺寸与状态外观

视觉控制入口：`side`。状态组合以公开属性和行为规范为准，不为 Gallery 虚构额外状态。

### Theme Token

实际消费：`Theme.bg-elevated`、`Theme.border-default`、`Theme.border-secondary`、`Theme.border-width`、`Theme.control-height-small`、`Theme.direction`、`Theme.drawer-width`、`Theme.fill-hover`、`Theme.font-size-title-small`、`Theme.font-weight-medium`、`Theme.icon-default`、`Theme.icon-size-x-small`、`Theme.overlay-mask`、`Theme.radius-small`、`Theme.space-2`、`Theme.space-4`、`Theme.text-primary`。不得在调用方通过任意颜色绕过这些语义 Token。

### 内容、布局与环境

遵循全局排版、密度、长文本、100%–200% 缩放、浅色/深色/高对比度和 LTR/RTL 规则；组件特有差异见本页组成结构及行为规范。

## 行为规范

### 状态与交互

支持 start/end 与 RTL 映射、遮罩关闭和 Escape 关闭请求；开关状态由宿主控制。

### 无障碍与本地化

遮罩区域和面板分别提供 region/groupbox 语义；标题与关闭名称由调用方本地化。

### 验证、宿主职责与限制

Gallery 提供打开入口。当前稳定 Slint 公开能力不提供通用焦点陷阱、所有者焦点恢复和平台级模态关系；宿主负责打开前后焦点。

全局约束：[`foundations.md`](../../foundations.md)、[`interaction.md`](../../interaction.md)、[`accessibility.md`](../../accessibility.md)、[`content-and-localization.md`](../../content-and-localization.md)。
