# Spinner

状态：已实现。源码：`ui/feedback/spinner.slint`。

## 用途与边界

用于局部短时加载。可量化任务使用 Progress，阻塞整个区域使用后续 LoadingOverlay；Spinner 不管理任务、超时或取消。

## 公开 API

`size: SpinnerSize = medium`（small/medium/large）、`indicator-color: color = Theme.color-primary`、`accessible-name: string = ""`。

## 状态与交互

只有 loading 展示状态，不接收指针、键盘或焦点。SVG 按 Token 着色并使用统一动效时钟；reduced-motion 时停止旋转但保留静态图形和语义。

## 无障碍与本地化

角色为 progress-indicator；必须提供说明当前等待对象的本地化名称。装饰性内部图像不重复进入可访问树。

## Gallery、测试与限制

Gallery“反馈”和 Button Loading 场景覆盖尺寸、主题和减少动效；通过编译与截图验收。软件渲染器可能不展示旋转变换，但静态回退保持可见，不增加第二套组件。遵循四份全局规范。
