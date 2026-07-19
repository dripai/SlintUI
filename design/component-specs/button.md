# Button

状态：已实现。源码：`ui/controls/button.slint`。

## 用途与边界

用于离散操作和可选的可切换操作。立即生效的布尔设置使用 `Switch`，多项互斥选择使用 `SegmentedControl`，导航链接后续使用 `Link`。

## 公开 API

- `text: string = ""`、`icon: image`、`variant: ButtonVariant = default`、`size: ControlSize = medium`。
- `enabled: bool = true`、`loading: bool = false`、`checkable: bool = false`、`checked: bool = false`、`keyboard-focusable: bool = true`。
- `accessible-name: string = text`、`clicked()`、`activate()`；variant 为 default/primary/danger/text/link。

## 状态与交互

支持 Default、Hover、Pressed、Focused、Disabled、Checked、Loading；不适用错误状态。点击或 Enter/Space 调用同一 `activate()`；禁用和 Loading 阻止状态变化及回调，checkable 在回调前更新 checked。

## 无障碍与本地化

角色为 button，暴露 enabled、checkable、checked 和默认动作。无文字按钮必须提供本地化名称；图标为装饰，不重复朗读。文本支持缩放，过长时省略，关键操作不得只靠图标或颜色。

## Gallery、测试与限制

Gallery“操作”页覆盖五种 variant、三种尺寸、Loading 和 Disabled；自动测试覆盖正常、禁用、IconButton、ToolButton 与 checked 单次转换。实现复用 `TouchArea`、`FocusScope` 和可访问回调，遵循四份全局规范。
