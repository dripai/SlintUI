# Label

状态：已实现。源码：`ui/primitives/label.slint`。

## 用途与边界

用于字段标签和普通说明标签。操作文本使用 Button/Link；补充说明由调用方组合 `Tooltip`，不在 Label 内隐式创建浮层。Slint 1.17.1 没有稳定的跨平台助记键关联 API，因此 P0 不承诺助记键。

## 公开 API

- 继承 `Text` 的 `text`、`wrap`、`overflow` 等属性，可由调用方选择换行或 `no-wrap + elide` 截断。
- `required: bool = false`、`muted: bool = false`。

## 状态与交互

没有独立 Hover、Pressed、Focused、Selected、Disabled 或 Loading 状态；Label 不接收输入。`required` 是供表单组合读取的标记，不自行改写文字；当前 `FormRow` 负责追加可见星号，校验状态由 FormRow 和输入组件表达。

## 无障碍与本地化

角色为 `text`，可访问名称来自可见文本。Placeholder 不替代 Label；Slint 当前无 required 可访问状态，宿主表单必须在说明或错误中表达约束。支持中文、英文、RTL、长文本和文本缩放。

## Gallery、测试与限制

Gallery 基础和表单场景覆盖普通、muted、required 组合环境；非交互组件无需状态转换测试。全局规则见 [`../foundations.md`](../foundations.md)、[`../accessibility.md`](../accessibility.md) 和 [`../content-and-localization.md`](../content-and-localization.md)。
